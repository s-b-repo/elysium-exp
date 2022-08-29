use crate::entity::{Entity, Player, PlayerRef};
use crate::State;
use elysium_math::Vec3;
use elysium_sdk::convar::Vars;
use elysium_sdk::entity::{MoveKind, Networkable, ObserverMode, Renderable};
use elysium_sdk::{Command, EntityList, HitGroup, Interfaces};
use std::arch::asm;

#[inline]
fn get_dir(movement: Vec3, forward: Vec3, right: Vec3) -> Vec3 {
    let x = forward.x * movement.x + right.x * movement.y;
    let y = forward.y * movement.x + right.y * movement.y;

    Vec3::from_xy(x, y)
}

#[inline]
fn fix_movement(command: &mut Command, wish_angle: Vec3) {
    let (mut wish_forward, mut wish_right, _wish_up) = wish_angle.angle_vector();
    let (mut curr_forward, mut curr_right, _curr_up) = command.view_angle.angle_vector();

    wish_forward.z = 0.0;
    wish_right.z = 0.0;
    curr_forward.z = 0.0;
    curr_right.z = 0.0;

    wish_forward = wish_forward.normalize();
    wish_right = wish_right.normalize();
    curr_forward = curr_forward.normalize();
    curr_right = curr_right.normalize();

    let wish_dir = get_dir(command.movement, wish_forward, wish_right);
    let curr_dir = get_dir(command.movement, curr_forward, curr_right);

    if wish_dir != curr_dir {
        let denom = curr_right.y * curr_forward.x - curr_right.x * curr_forward.y;

        command.movement.x = (wish_dir.x * curr_right.y - wish_dir.y * curr_right.x) / denom;
        command.movement.y = (wish_dir.y * curr_forward.x - wish_dir.x * curr_forward.y) / denom;
    }
}

#[inline]
fn calculate_angle(src: Vec3, dst: Vec3) -> Vec3 {
    let delta = src - dst;
    let hypot = (delta.x * delta.x + delta.y * delta.y).sqrt();

    let x = (delta.z / hypot).atan().to_degrees();
    let mut y = (delta.y / delta.x).atan().to_degrees();
    let z = 0.0;

    if delta.x >= 0.0 {
        y += 180.0;
    }

    Vec3::from_xyz(x, y, z)
}

#[inline]
unsafe fn do_create_move(command: &mut Command, local: PlayerRef<'_>, send_packet: &mut bool) {
    let state = State::get();
    let vars = state.vars.as_ref().unwrap();
    let mut local_vars = &mut state.local;
    let Interfaces { entity_list, .. } = state.interfaces.as_ref().unwrap();
    let globals = state.globals.as_ref().unwrap();
    let players = &mut state.players;

    let do_attack = command.in_attack();
    let do_duck = command.in_duck();
    let do_jump = command.in_jump();
    let on_ground = local.flags().on_ground();
    let was_attacking = local_vars.was_attacking;
    let was_jumping = local_vars.was_jumping;
    let side = if command.command % 2 != 0 { 1.0 } else { -1.0 };

    local_vars.was_attacking = do_attack;
    local_vars.was_jumping = do_jump;

    if do_attack && was_attacking {
        command.attack(false);
        local_vars.was_attacking = false;
    }

    if do_jump {
        if on_ground {
            command.duck(false);
        } else {
            command.jump(false);
        }
    }

    if !on_ground {
        // don't do anything fancy whilest on a ladder or noclipping
        if !matches!(local.move_kind(), MoveKind::NoClip | MoveKind::Ladder) {
            let velocity = local.velocity();
            let magnitude = velocity.magnitude2d();
            let ideal_strafe = (15.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0);
            let mut wish_angle = command.view_angle;
            let strafe_dir = command.movement.to_dir();
            let strafe_dir_yaw_offset = strafe_dir.y.atan2(strafe_dir.x).to_degrees();

            wish_angle.y -= strafe_dir_yaw_offset;

            let mut wish_angle = wish_angle.sanitize_angle();
            let yaw_delta = libm::remainderf(wish_angle.y - local_vars.old_yaw, 360.0);
            let abs_yaw_delta = yaw_delta.abs();

            local_vars.old_yaw = wish_angle.y;

            let horizontal_speed = vars.horizontal_speed.read();

            if abs_yaw_delta <= ideal_strafe || abs_yaw_delta >= 30.0 {
                let velocity_dir = Vec3::vector_angle(velocity);
                let velocity_yaw_delta = libm::remainderf(wish_angle.y - velocity_dir.y, 360.0);
                let retrack = (30.0 / magnitude).atan().to_degrees().clamp(0.0, 90.0) * 2.0;

                if velocity_yaw_delta <= retrack || magnitude <= 15.0 {
                    if -retrack <= velocity_yaw_delta || magnitude <= 15.0 {
                        wish_angle.y += side * ideal_strafe;
                        command.movement.y = horizontal_speed * side;
                    } else {
                        wish_angle.y = velocity_dir.y - retrack;
                        command.movement.y = horizontal_speed;
                    }
                } else {
                    wish_angle.y = velocity_dir.y + retrack;
                    command.movement.y = -horizontal_speed;
                }
            } else if yaw_delta > 0.0 {
                command.movement.y = -horizontal_speed;
            } else if yaw_delta < 0.0 {
                command.movement.y = horizontal_speed
            }

            command.movement.x = 0.0;

            fix_movement(command, wish_angle);
        }
    }

    if state.local.anti_aim {
        if state.fake_lag != 0 {
            *send_packet = command.command % state.fake_lag as i32 == 0;
        }

        // don't do anything fancy whilest on a ladder or noclipping
        if !matches!(local.move_kind(), MoveKind::NoClip | MoveKind::Ladder) {
            let (x, y, z) = command.view_angle.to_tuple();
            let view_angle = if *send_packet {
                let x = state.pitch.apply(x);
                let y = state.yaw.apply(y);
                let z = state.roll.apply(z);

                (x, y, z)
            } else {
                let x = state.fake_pitch.apply(x);
                let y = state.fake_yaw.apply(y);
                let z = state.fake_roll.apply(z);

                (x, y, z)
            };

            command.view_angle = Vec3::from_tuple(view_angle);
        }
    }

    let player_iter = entity_list
        .player_range()
        .flat_map(|index| Some((index, PlayerRef::from_raw(entity_list.entity(index))?)));

    for (index, player) in player_iter {
        println!("{index} {player:?}");
    }

    if do_attack {
        command.view_angle = state.view_angle;
    }

    command.fast_duck(true);

    /*match command.command % 14 {
        0..=7 => {
            *send_packet = true;
            command.duck(true);
        }
        7..=14 => {
            *send_packet = false;
            command.duck(false);
        }
        _ => {},
    }*/

    fix_movement(command, state.view_angle);

    if state.anti_untrusted {
        command.view_angle = command.view_angle.sanitize_angle();
    }
}

/// `CreateMove` hook.
pub unsafe extern "C" fn create_move(
    this: *const u8,
    input_sample_time: f32,
    command: &mut Command,
) -> bool {
    let rbp: *mut *mut bool;

    core::arch::asm!("mov {}, rbp", out(reg) rbp, options(nostack));

    let send_packet = &mut *(*rbp).sub(24);

    //
    let state = State::get();
    let create_move_original = state.hooks.create_move.unwrap();
    let globals = state.globals.as_ref().unwrap();

    (create_move_original)(this, input_sample_time, command);

    if command.tick_count == 0 {
        return false;
    }

    let mut local = PlayerRef::from_raw(state.local.player).unwrap();

    /*println!("active_weapon = {:?}", local.active_weapon());
    println!("aim_punch = {:?}", local.aim_punch());
    println!("armor_value = {:?}", local.armor_value());
    //println!("damage_modifier = {:?}", local.damage_modifier());
    println!("eye_offset = {:?}", local.eye_offset());
    println!("eye_origin = {:?}", local.eye_origin());
    println!("flags = {:?}", local.flags());
    println!("has_helmet = {:?}", local.has_helmet());
    println!("is_defusing = {:?}", local.is_defusing());
    println!("is_scoped = {:?}", local.is_scoped());
    println!("lower_body_yaw = {:?}", local.lower_body_yaw());
    println!("move_kind = {:?}", local.move_kind());
    println!("observer_mode = {:?}", local.observer_mode());*/

    // don't mess with input if you are spectating
    if local.observer_mode() != ObserverMode::None {
        return false;
    }

    do_create_move(command, local, send_packet);

    let mut local = PlayerRef::from_raw(state.local.player).unwrap();
    let time = globals.current_time;

    if *send_packet {
        let mut bones = &mut state.local.bones;
        let view_angle = local.view_angle();

        local.set_view_angle(command.view_angle);
        local.setup_bones(&mut bones[..128], 0x00000100, time);
        local.setup_bones(&mut bones[..128], 0x000FFF00, time);
        local.set_view_angle(view_angle);

        state.local.view_angle = command.view_angle;
        state.local.time = globals.current_time;
    } else {
        let mut bones = &mut state.local.fake_bones;
        let view_angle = local.view_angle();

        local.set_view_angle(command.view_angle);
        local.setup_bones(&mut bones[..128], 0x00000100, time);
        local.setup_bones(&mut bones[..128], 0x000FFF00, time);
        local.set_view_angle(view_angle);
    }

    println!("{:?}", crate::state::is_record_valid(globals.current_time));

    false
}

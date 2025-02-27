use crate::entity::{Entity, Player, PlayerRef};
use crate::{state, State};
use elysium_math::Matrix3x4;
use elysium_sdk::material;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const PURPLE: [f32; 4] = [0.4, 0.0, 1.0, 0.4];
const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 0.5];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.5];

#[inline]
unsafe fn draw_model_inner(
    model_render: &mut ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) -> Option<()> {
    let state = State::get();
    let draw_model_original = state.hooks.draw_model?;
    let interfaces = state.interfaces.as_ref()?;
    let entity_list = &interfaces.entity_list;
    let model_info = &interfaces.model_info;
    let materials = &interfaces.materials;
    let local = PlayerRef::from_raw(state.local.player)?;

    let flat = state::material::FLAT.load_unchecked();
    let glow = state::material::GLOW.load_unchecked();

    let info = info.as_ref()?;
    let name = info.name(&model_info)?;

    if name.starts_with("models/player") {
        let index = info.entity_index;
        let player = PlayerRef::from_raw(entity_list.entity(index))?;

        if index == local.index() {
            flat.set_rgba(BLACK);
            glow.set_rgba(PURPLE);

            flat.set_flag(material::Flag::IGNORE_Z, false);
            glow.set_flag(material::Flag::IGNORE_Z, false);

            model_render.override_material(flat);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.override_material(glow);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.reset_material();
        } else {
            let (rgba, ignore_z) = match (player.is_enemy(), player.flags().is_bot()) {
                (false, true) => (PURPLE, false),
                (true, false) => (RED, true),
                (true, true) => (ORANGE, true),
                _ => ([1.0, 1.0, 1.0, 0.5], false),
            };

            flat.set_rgba(BLACK);
            glow.set_rgba(rgba);

            flat.set_flag(material::Flag::IGNORE_Z, ignore_z);
            glow.set_flag(material::Flag::IGNORE_Z, ignore_z);

            model_render.override_material(flat);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.override_material(glow);
            (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            model_render.reset_material();
        }
    } else if name.starts_with("models/weapons/v_") {
        flat.set_flag(material::Flag::IGNORE_Z, false);
        glow.set_flag(material::Flag::IGNORE_Z, false);

        flat.set_rgba(BLACK);
        glow.set_rgba(PURPLE);

        if local.is_scoped() {
            flat.set_alpha(0.2);
            glow.set_alpha(0.2);
        }
        /*glow.set_flag(material::Flag::IGNORE_Z, false);

        if let Some(class) = weapon.client_class() {
            match class.entity_id {
                EntityId::CWeaponAug => {
                    glow.set_rgba(PURPLE);

                    model_render.override_material(glow);
                    (draw_model_original)(
                        model_render,
                        context,
                        draw_state,
                        info,
                        bone_to_world,
                    );
                    model_render.reset_material();

                    return Some(());
                }
                _ => {}
            }
        }*/

        model_render.override_material(flat);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.override_material(glow);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    } else {
        flat.set_flag(material::Flag::IGNORE_Z, false);
        glow.set_flag(material::Flag::IGNORE_Z, false);

        flat.set_rgba(BLACK);
        glow.set_rgba(PURPLE);

        model_render.override_material(flat);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.override_material(glow);
        (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
        model_render.reset_material();
    }

    Some(())
}

/// `DrawModelExecute` hook.
pub unsafe extern "C" fn draw_model(
    model_render: &mut ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    draw_model_inner(model_render, context, draw_state, info, bone_to_world);
}

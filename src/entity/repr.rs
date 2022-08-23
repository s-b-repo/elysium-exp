use super::PlayerRef;
use crate::{Networked, State};
use cake::ffi::VTablePad;
use elysium_math::{Matrix3x4, Vec3};
use elysium_sdk::client::Class;
use elysium_sdk::entity::{MoveKind, Networkable, ObserverMode, PlayerFlags, Renderable, Team};
use elysium_sdk::model::Model;
use elysium_sdk::{object_validate, vtable_validate, HitGroup, WeaponInfo};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::{Bound, RangeBounds, RangeInclusive};

pub use exposure::Exposure;

#[repr(C)]
struct VTable {
    _pad0: VTablePad<12>,
    origin: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const Vec3,
    _pad1: VTablePad<98>,
    set_model_index: unsafe extern "thiscall" fn(this: *mut EntityRepr, index: i32),
    _pad2: VTablePad<10>,
    attachment:
        unsafe extern "thiscall" fn(this: *const EntityRepr, index: i32, origin: *mut Vec3) -> bool,
    _pad3: VTablePad<5>,
    team: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Team,
    _pad4: VTablePad<38>,
    health: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad5: VTablePad<40>,
    is_alive: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad6: VTablePad<1>,
    is_player: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad7: VTablePad<7>,
    is_weapon: unsafe extern "thiscall" fn(this: *const EntityRepr) -> bool,
    _pad8: VTablePad<112>,
    active_weapon: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const EntityRepr,
    _pad9: VTablePad<16>,
    eye_pos: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Vec3,
    _pad10: VTablePad<1>,
    weapon_sub_kind: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad11: VTablePad<6>,
    observer_mode: unsafe extern "thiscall" fn(this: *const EntityRepr) -> ObserverMode,
    observer_target: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const EntityRepr,
    _pad12: VTablePad<50>,
    aim_punch: unsafe extern "thiscall" fn(this: *const EntityRepr) -> Vec3,
    _pad13: VTablePad<62>,
    draw_crosshair: unsafe extern "thiscall" fn(this: *const EntityRepr),
    _pad14: VTablePad<48>,
    spread: unsafe extern "thiscall" fn(this: *const EntityRepr) -> f32,
    _pad15: VTablePad<1>,
    weapon_kind: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad16: VTablePad<5>,
    weapon_data: unsafe extern "thiscall" fn(this: *const EntityRepr) -> *const WeaponInfo,
    _pad17: VTablePad<6>,
    muzzle_attachment_index_1st:
        unsafe extern "thiscall" fn(this: *const EntityRepr, view_model: *const EntityRepr) -> i32,
    muzzle_attachment_index_3rd: unsafe extern "thiscall" fn(this: *const EntityRepr) -> i32,
    _pad18: VTablePad<13>,
    inaccuracy: unsafe extern "thiscall" fn(this: *const EntityRepr) -> f32,
    update_accuracy_penalty: unsafe extern "thiscall" fn(this: *const EntityRepr),
}

vtable_validate! {
    origin => 12,
    set_model_index => 111,
    attachment => 122,
    team => 128,
    health => 167,
    is_alive => 208,
    is_player => 210,
    is_weapon => 218,
    active_weapon => 331,
    eye_pos => 348,
    weapon_sub_kind => 350,
    observer_mode => 357,
    observer_target => 358,
    aim_punch => 409,
    draw_crosshair => 472,
    spread => 521,
    weapon_kind => 523,
    weapon_data => 529,
    muzzle_attachment_index_1st => 536,
    muzzle_attachment_index_3rd => 537,
    inaccuracy => 551,
    update_accuracy_penalty => 552,
}

#[repr(C)]
pub struct EntityRepr {
    vtable: &'static VTable,
    pub renderable: Renderable,
    pub networkable: Networkable,
}

object_validate! {
    EntityRepr;
    vtable => 0,
    renderable => 8,
    networkable => 16,
}

// generic
impl EntityRepr {
    #[inline]
    fn as_ptr(&self) -> *const EntityRepr {
        self.entity.as_ptr() as *const EntityRepr
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut EntityRepr {
        self.entity.as_ptr() as *mut EntityRepr
    }

    #[inline]
    pub fn networked<T, F>(&self, f: F) -> *const T
    where
        F: Fn(&Networked) -> usize,
    {
        let state = State::get();
        let offset = f(&state.networked);

        unsafe { self.as_ptr().cast::<T>().byte_add(offset) }
    }

    #[inline]
    pub fn networked_mut<T, F>(&mut self, f: F) -> *mut T
    where
        F: Fn(&Networked) -> usize,
    {
        let state = State::get();
        let offset = f(&state.networked);

        unsafe { self.as_mut_ptr().cast::<T>().byte_add(offset) }
    }

    #[inline]
    pub fn attachment(&self, index: i32) -> Option<Vec3> {
        let mut origin = MaybeUninit::uninit();

        unsafe {
            (self.vtable.attachment)(self, index, origin.as_mut_ptr())
                .then(|| MaybeUninit::assume_init(origin))
        }
    }

    #[inline]
    pub fn client_class(&self) -> Option<&Class> {
        self.networkable.client_class().cast().as_ref()
    }

    #[inline]
    pub fn health(&self) -> i32 {
        unsafe { (self.vtable.health)(self) }
    }

    #[inline]
    pub fn is_alive(&self) -> bool {
        unsafe { (self.vtable.is_alive)(self) }
    }

    #[inline]
    pub fn is_dormant(&self) -> bool {
        self.networkable.is_dormant()
    }

    #[inline]
    pub fn is_player(&self) -> bool {
        unsafe { (self.vtable.is_player)(self) }
    }

    #[inline]
    pub fn is_weapon(&self) -> bool {
        unsafe { (self.vtable.is_weapon)(self) }
    }

    #[inline]
    pub fn index(&self) -> i32 {
        self.networkable.index()
    }

    #[inline]
    pub fn model(&self) -> Option<&Model> {
        self.renderable.model().cast().as_ref()
    }

    #[inline]
    pub fn origin(&self) -> Vec3 {
        unsafe { *(self.vtable.origin)(self) }
    }

    #[inline]
    fn render_mode_address(&self) -> *const u8 {
        self.networked(|networked| networked.base_entity.render_mode)
    }

    #[inline]
    pub fn set_model_index(&mut self, index: i32) {
        unsafe { (self.vtable.set_model_index)(self, index) }
    }

    #[inline]
    pub fn setup_bones(&self, bones: &mut [Matrix3x4], mask: i32, time: f32) -> bool {
        self.renderable.setup_bones(bones, mask, time)
    }

    #[inline]
    pub fn should_draw(&self) -> bool {
        self.renderable.should_draw()
    }
}

// player
impl EntityRepr {
    #[inline]
    pub fn aim_punch(&self) -> Vec3 {
        unsafe { (self.vtable.aim_punch)(self) }
    }

    #[inline]
    pub fn active_weapon(&self) -> *const EntityRepr {
        unsafe { (self.vtable.active_weapon)(self) }
    }

    #[inline]
    pub fn armor(&self) -> i32 {
        unsafe {
            self.networked(|networked| networked.player.armor)
                .read_unaligned()
        }
    }

    #[inline]
    pub fn eye_offset(&self) -> Vec3 {
        unsafe {
            let view_offset = self
                .networked(|networked| networked.base_player.view_offset)
                .read_unaligned();

            // zero view offset fix
            if view_offset.is_zero() {
                let z = if self.flags().ducking() { 46.0 } else { 64.0 };

                Vec3::from_xyz(0.0, 0.0, z)
            } else {
                view_offset
            }
        }
    }

    #[inline]
    pub fn eye_origin(&self) -> Vec3 {
        self.offset() + self.view_offset()
    }

    // TODO: check if this is better than above
    #[inline]
    pub fn eye_origin_alt(&self) -> Vec3 {
        unsafe { (self.vtable.eye_pos)(self) }
    }

    #[inline]
    pub fn damage_modifier(&self, group: HitGroup, weapon_armor_ratio: f32) -> f32 {
        let mut modifier = group.damage_modifier();

        if self.armor() > 0 {
            if group.is_head() && self.has_helmet() {
                modifier *= weapon_armor_ratio * 0.5;
            }
        }

        modifier
    }

    #[inline]
    fn is_dead_address(&self) -> *const u8 {
        self.networked(|networked| networked.base_player.is_dead)
    }

    #[inline]
    pub fn flags(&self) -> PlayerFlags {
        unsafe {
            let flags = self
                .networked(|networked| networked.player.flags)
                .read_unaligned();

            PlayerFlags::new(flags)
        }
    }

    #[inline]
    pub fn has_helmet(&self) -> bool {
        unsafe {
            self.networked(|networked| networked.player.has_helmet)
                .read_unaligned()
        }
    }

    #[inline]
    pub fn is_defusing(&self) -> bool {
        unsafe {
            self.networked(|networked| networked.player.is_defusing)
                .read_unaligned()
        }
    }

    #[inline]
    pub fn is_scoped(&self) -> bool {
        unsafe {
            self.networked(|networked| networked.player.is_scoped)
                .read_unaligned()
        }
    }

    #[inline]
    pub fn lower_body_yaw(&self) -> i32 {
        unsafe {
            self.networked(|networked| networked.player.lower_body_yaw)
                .read_unaligned()
        }
    }

    #[inline]
    pub fn observer_mode(&self) -> ObserverMode {
        unsafe { (self.vtable.observer_mode)(self) }
    }

    #[inline]
    pub fn observer_target(&self) -> Option<PlayerRef> {
        unsafe {
            let observer = (self.vtable.observer_target)(self);

            PlayerRef::from_raw(observer)
        }
    }

    #[inline]
    pub fn move_kind(&self) -> MoveKind {
        unsafe {
            self.render_mode_address()
                .byte_add(1)
                .cast()
                .read_unaligned()
        }
    }

    #[inline]
    pub unsafe fn set_view_angle(&mut self, angle: Vec3) {
        self.is_dead_address()
            .byte_add(4)
            .cast::<Vec3>()
            .cast_mut()
            .write_unaligned(angle)
    }

    #[inline]
    pub fn team(&self) -> Team {
        unsafe { (self.vtable.team)(self) }
    }

    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            self.is_dead_address()
                .byte_add(4)
                .cast::<Vec3>()
                .read_unaligned()
        }
    }

    #[inline]
    pub fn velocity(&self) -> Vec3 {
        unsafe {
            self.networked(|networked| networked.base_player.velocity)
                .read_unaligned()
        }
    }

    #[inline]
    pub fn velocity_magnitude(&self) -> f32 {
        self.velocity().magnitude()
    }
}

// fog
impl EntityRepr {
    /// Returns the fog’s clip distance (far-Z).
    #[inline]
    pub fn clip_distance(&self) -> f32 {
        unsafe {
            self.networked(|networked| networked.fog.far_z)
                .read_unaligned()
        }
    }

    /// Whether fog is enabled.
    #[inline]
    pub fn enabled(&self) -> bool {
        unsafe {}
    }

    /// Returns the fog’s range (start and end distance).
    #[inline]
    pub fn range(&self) -> Option<RangeInclusive<f32>> {
        unsafe {
            self.networked(|networked| networked.fog.is_enabled)
                .read_unaligned()
                .then(|| {
                    let start = self
                        .networked(|networked| networked.fog.start)
                        .read_unaligned();

                    let end = self
                        .networked(|networked| networked.fog.end)
                        .read_unaligned();

                    start..=end
                })
        }
    }

    /// Returns the fog’s color (rgb) and density (alpha).
    #[inline]
    pub fn rgba(&self) -> (u8, u8, u8, f32) {
        unsafe {
            let rgb = self
                .networked(|networked| networked.fog.color_primary)
                .read_unaligned();

            let alpha = self
                .networked(|networked| networked.fog.density)
                .read_unaligned();

            let [r, g, b, _] = rgb.to_ne_bytes();

            (r, g, b, alpha)
        }
    }

    /// Set the fog’s clip distance (far-Z).
    #[inline]
    pub fn set_clip_distance(&mut self, distance: f32) {
        unsafe {
            self.networked_mut(|networked| networked.fog.far_z)
                .write_unaligned(distance)
        }
    }

    /// Set the fog’s range (start and end distance).
    #[inline]
    pub fn set_range(&self, range: Option<RangeInclusive<f32>>) {
        unsafe {
            self.networked_mut(|networked| networked.fog.is_enabled)
                .write_unaligned(range.is_some());

            if let Some(range) = range {
                let RangeInclusive { start, end } = range;

                self.networked_mut(|networked| networked.fog.start)
                    .write_unaligned(start);

                self.networked_mut(|networked| networked.fog.end)
                    .write_unaligned(end);
            }
        }
    }

    /// Set the fog’s color (rgb) and density (alpha).
    #[inline]
    pub fn set_rgba(&self, rgba: (u8, u8, u8, f32)) {
        let (r, g, b, alpha) = rgba;
        let rgb = i32::from_ne_bytes([r, g, b, 0]);

        unsafe {
            self.networked_mut(|networked| networked.fog.color_primary)
                .write_unaligned(rgb);

            self.networked_mut(|networked| networked.fog.density)
                .write_unaligned(alpha);
        }
    }
}

mod exposure {
    use std::mem;
    use std::ops::{Bound, RangeBounds};

    #[derive(Clone, Copy)]
    pub struct Exposure {
        start: Bound<u16>,
        end: Bound<u16>,
    }

    impl<R> From<R> for Exposure
    where
        R: RangeBounds<u16>,
    {
        #[inline]
        fn from(range: R) -> Self {
            let start = range.start_bound().map(mem::copy);
            let end = range.end_bound().map(mem::copy);

            Self { start, end }
        }
    }
}

// tonemap
impl EntityRepr {
    /// Returns the tonemap's bloom effect setting.
    #[inline]
    pub fn bloom(&self) -> Option<f32> {
        unsafe {
            let enabled: bool = self
                .networked(|networked| networked.tonemap.enable_bloom_scale)
                .read_unaligned();

            enabled.then(|| {
                self.networked(|networked| networked.tonemap.bloom_scale)
                    .read_unaligned()
            })
        }
    }

    /// Returns the tonemap's bloom effect setting.
    #[inline]
    pub fn exposure(&self) -> Option<Exposure> {
        unsafe {
            let min_enabled = self
                .networked(|networked| networked.tonemap.enable_min_exposure)
                .read_unaligned();

            let max_enabled = self
                .networked(|networked| networked.tonemap.enable_max_exposure)
                .read_unaligned();

            let min = self
                .networked(|networked| networked.tonemap.min_exposure)
                .read_unaligned();

            let max = self
                .networked(|networked| networked.tonemap.max_exposure)
                .read_unaligned();

            match (min_enabled, max_enabled) {
                (true, true) => Some(Exposure::from(min..=max)),
                (true, false) => Some(Exposure::from(min..)),
                (false, true) => Some(Exposure::from(..=min)),
                (false, false) => None,
            }
        }
    }

    /// Returns the tonemap's bloom effect setting.
    #[inline]
    pub fn set_bloom(&mut self, scale: Option<f32>) {
        let enabled = scale.is_some();

        unsafe {
            self.networked_mut(|networked| networked.tonemap.enable_bloom_scale)
                .write_unaligned(enabled);

            if let Some(scale) = scale {
                self.networked_mut(|networked| networked.tonemap.bloom_scale)
                    .write_unaligned(scale);
            }
        }
    }

    /// Sets the tonemap's bloom effect setting.
    #[inline]
    pub fn set_exposure<R: RangeBounds<u16>>(&mut self, exposure: Option<R>) {
        fn map_bound(bound: Bound<u16>) -> Option<i32> {
            match bound {
                Bound::Included(bound) => Some(bound as i32),
                Bound::Excluded(bound) => Some(bound.saturating_sub(1) as i32),
                Bound::Unbounded => None,
            }
        }

        let maybe_exposure = exposure.map(Exposure::from);
        let (start, end) = match maybe_exposure {
            Some(exposure) => {
                let Exposure { start, end } = exposure.map(Exposure::from);
                let start = map_bound(start);
                let end = map_bound(end);

                (start, end)
            }
            None => (None, None),
        };

        unsafe {
            self.networked_mut(|networked| networked.tonemap.enable_min_exposure)
                .write_unaligned(start.is_some());

            self.networked_mut(|networked| networked.tonemap.enable_max_exposure)
                .write_unaligned(end.is_some());

            let start = start.unwrap_or(0);
            let end = end.unwrap_or(i32::MAX);

            self.networked_mut(|networked| networked.tonemap.min_exposure)
                .write_unaligned(start);

            self.networked_mut(|networked| networked.tonemap.max_exposure)
                .write_unaligned(end);
        }
    }
}

// weapon
impl EntityRepr {
    #[inline]
    pub fn magazine(&self) -> Option<u16> {
        let magazine: i32 = *self.networked(|networked| networked.base_weapon.magazine);

        if magazine < 0 {
            None
        } else {
            Some(magazine as u16)
        }
    }

    #[inline]
    pub fn next_attack_time(&self) -> &mut f32 {
        self.networked(|networked| networked.base_weapon.next_attack_time)
    }

    #[inline]
    pub fn revolver_cock_time(&self) -> Option<f32> {
        let time: f32 = *self.networked(|networked| networked.weapon.revolver_cock_time);

        if time > 3.4028235e+38 {
            None
        } else {
            Some(time)
        }
    }

    #[inline]
    pub fn draw_crosshair(&self) {
        unsafe { (self.vtable.draw_crosshair)(self) }
    }

    #[inline]
    pub fn spread(&self) -> f32 {
        unsafe { (self.vtable.spread)(self) }
    }

    #[inline]
    pub fn weapon_kind(&self) -> i32 {
        unsafe { (self.vtable.weapon_kind)(self) }
    }

    #[inline]
    pub fn weapon_data(&self) -> *const WeaponInfo {
        unsafe { (self.vtable.weapon_data)(self) }
    }

    #[inline]
    pub fn muzzle_attachment_index_1st(&self, view_model: *const EntityRepr) -> i32 {
        unsafe { (self.vtable.muzzle_attachment_index_1st)(self, view_model) }
    }

    #[inline]
    pub fn muzzle_attachment_index_3rd(&self) -> i32 {
        unsafe { (self.vtable.muzzle_attachment_index_3rd)(self) }
    }

    #[inline]
    pub fn inaccuracy(&self) -> f32 {
        unsafe { (self.vtable.inaccuracy)(self) }
    }

    #[inline]
    pub fn update_accuracy_penalty(&self) {
        unsafe { (self.vtable.update_accuracy_penalty)(self) }
    }

    #[inline]
    pub fn weapon_sub_kind(&self) -> i32 {
        unsafe { (self.vtable.weapon_sub_kind)(self) }
    }
}

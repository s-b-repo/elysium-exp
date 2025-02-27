use cake::ffi::BytePad;

pub use engine::UIEngine;
pub use panel::UIPanel;
pub use panorama_engine::PanoramaUIEngine;

mod engine;
mod panel;
mod panorama_engine;

/// Panorama Event Registration.
#[repr(C)]
pub struct PanoramaEventRegistration {
    pub args_len: i32,
    _pad0: BytePad<4>,
    pub make_event: unsafe extern "thiscall" fn(this: *const ()) -> *const (),
    pub create_event_from_string: unsafe extern "thiscall" fn(
        this: *const (),
        args: *const u8,
        result: *const *const u8,
    ) -> *const (),
    _pad1: BytePad<48>,
}

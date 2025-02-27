//! Assembly patterns within libraries.

use core::mem;
use elysium_pattern::Pattern;
use elysium_sdk::LibraryKind;

pub const ANIMATION_LAYERS: Pattern<80> =
    Pattern::new("55 48 89 E5 41 56 41 55 41 89 F5 41 54 53 48 89 FB 8B");

pub const ANIMATION_STATE: Pattern<84> =
    Pattern::new("55 48 89 E5 53 48 89 FB 48 83 EC 28 48 8B 05 ?? ?? ?? ?? 48 8B 00");

pub const SAVE_DATA: Pattern<108> =
    Pattern::new("55 48 89 E5 41 57 41 89 CF 41 56 41 55 41 89 D5 41 54 53 48 89 FB 48 81 EC");

pub const RESTORE_DATA: Pattern<36> = Pattern::new("E9 ?? ?? ?? ?? 90 55 48 63 F6");

pub const ON_POST_RESTORE_DATA: Pattern<60> =
    Pattern::new("55 BE ?? ?? ?? ?? 48 89 E5 41 54 53 48 89 FB E8");

/// xref `"CL_Move"` in `VPROF` macro
///
/// [engine/cl_main.cpp](https://github.com/VSES/SourceEngine2007/blob/master/se2007/engine/cl_main.cpp)
pub const CL_MOVE: Pattern<132> = Pattern::new(
    "55 48 89 E5 41 57 41 56 41 89 FE 41 55 41 54 53 48 81 EC 98 01 00 00 F3 0F 11 85 5C FE FF FF",
);

/// xref `CL_Move`
///
/// [engine/host.cpp](https://github.com/VSES/SourceEngine2007/blob/master/se2007/engine/host.cpp)
pub const HOST_RUN_FRAME_INPUT: Pattern<164> =
    Pattern::new("55 48 89 E5 41 57 66 41 0F 7E C7 41 56 41 55 41 89 FD 41 54 53 48 83 EC 08 48 8B 1D C8 25 94 00 44 8B 83 0C 10 00 00");

/// xref `"WriteUsercmd: from=%d to=%d\"`
///
/// [game/shared/usercmd.cpp](https://github.com/VSES/SourceEngine2007/blob/master/se2007/game/shared/usercmd.cpp)
pub const WRITE_USER_COMMAND: Pattern<68> =
    Pattern::new("55 48 89 E5 41 56 41 55 4C 8D 35 B1 19 17 02");

/// xref `WriteUsercmd`
pub const WRITE_USER_COMMAND_DELTA_TO_BUFFER: Pattern<72> =
    Pattern::new("55 48 8D 05 38 BC 68 01 41 89 F2 48 89 E5 41 57");

pub const VDF_FROM_BYTES: Pattern<44> = Pattern::new("E8 ?? ?? ?? ?? 48 89 DF 48 89 45 E0");

#[inline]
pub fn get<const N: usize>(library: LibraryKind, pattern: &Pattern<N>) -> Option<&'static [u8]> {
    let name = library.path();

    println!("elysium | find pattern {pattern:?} in {name}");

    let module = unsafe { link::load_module(library.path()).ok()? };

    // SAFETY: does this fucking look safe to you?
    let bytes: &'static [u8] = unsafe { mem::transmute(module.bytes()) };

    println!("bytes = {:?}", bytes.len());

    pattern
        .regex()
        .find(bytes)
        .map(|found| unsafe { bytes.get_unchecked(found.start()..) })
}

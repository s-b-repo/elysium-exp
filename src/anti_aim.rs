//! Anti-aim implementation.

use core::fmt;
use core::ops::RangeInclusive;
use elysium_math::Vec3;
use rand::Rng;

#[inline]
pub(crate) fn random(range: RangeInclusive<f32>) -> f32 {
    let mut random = rand::thread_rng();

    random.gen_range(range)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pitch {
    Default,
    Up,
    Down,
}

impl Pitch {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Pitch::Default => "Default",
            Pitch::Up => "Up",
            Pitch::Down => "Down",
        }
    }
}

impl fmt::Display for Pitch {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.as_str(), fmt)
    }
}

#[derive(Debug)]
pub struct AntiAim {
    pub enabled: bool,
    pub pitch: Pitch,
    pub yaw_jitter: bool,
    pub yaw_offset: f32,
    pub roll: bool,
}

impl AntiAim {
    #[inline]
    pub const fn new() -> Self {
        Self {
            enabled: false,
            pitch: Pitch::Default,
            yaw_jitter: false,
            yaw_offset: 0.0,
            roll: false,
        }
    }

    #[inline]
    pub fn apply(&self, side: bool, view_angle: Vec3) -> Vec3 {
        if !self.enabled {
            return view_angle;
        }

        let side = if side { -1.0 } else { 1.0 };
        let (x, y, z) = view_angle.to_tuple();

        let x = match self.pitch {
            Pitch::Up => -89.0,
            Pitch::Down => 89.0,
            _ => x,
        };

        let desync = 58.0 * side;
        let y = y
            + self.yaw_offset
            + if self.yaw_jitter {
                (desync - 15.0) + random(0.0..=15.0)
            } else {
                desync
            };

        let z = if self.roll { 50.0 * side } else { z };

        Vec3::from_xyz(x, y, z)
    }
}

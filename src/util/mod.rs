use crate::prelude::*;
use bevy::ecs::system::Resource;

mod macros;
pub use macros::*;

pub fn remove_resource<R: Resource>(mut commands: Commands) {
    commands.remove_resource::<R>();
}

pub fn despawn_all_with<C: Component>(mut commands: Commands, query: Query<Entity, With<C>>) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

///////////////////
/// Color Mods
///////////////////
pub trait ColorMod {
    fn new_grey(value: u8) -> Self;

    fn saturating_scalar_mul_div(self, numerator: u32, denominator: u32) -> Self;
}

impl ColorMod for RGB {
    fn new_grey(value: u8) -> Self { RGB::from_u8(value, value, value) }

    fn saturating_scalar_mul_div(self, numerator: u32, denominator: u32) -> Self {
        const fn single_channel(channel: u8, numerator: u32, denominator: u32) -> u8 {
            let as_u32 = ((channel as u32) * (numerator)) / denominator;
            if as_u32 > ::std::u8::MAX as u32 {
                ::std::u8::MAX
            } else {
                as_u32 as u8
            }
        }

        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;

        Self {
            r: f32::from(single_channel(r, numerator, denominator)) / 255.0,
            g: f32::from(single_channel(g, numerator, denominator)) / 255.0,
            b: f32::from(single_channel(b, numerator, denominator)) / 255.0,
        }
    }
}

impl ColorMod for RGBA {
    fn new_grey(value: u8) -> Self { RGBA::from_u8(value, value, value, 255) }

    fn saturating_scalar_mul_div(self, numerator: u32, denominator: u32) -> Self {
        const fn single_channel(channel: u8, numerator: u32, denominator: u32) -> u8 {
            let as_u32 = ((channel as u32) * (numerator)) / denominator;
            if as_u32 > ::std::u8::MAX as u32 {
                ::std::u8::MAX
            } else {
                as_u32 as u8
            }
        }

        let r = (self.r * 255.0) as u8;
        let g = (self.g * 255.0) as u8;
        let b = (self.b * 255.0) as u8;

        Self {
            r: f32::from(single_channel(r, numerator, denominator)) / 255.0,
            g: f32::from(single_channel(g, numerator, denominator)) / 255.0,
            b: f32::from(single_channel(b, numerator, denominator)) / 255.0,
            a: self.a,
        }
    }
}

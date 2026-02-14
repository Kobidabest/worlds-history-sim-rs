use rand::{rngs::SmallRng, Rng};
use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub fn cartesian_coordinates(alpha: f32, beta: f32, radius: f32) -> Vec3 {
    let mut beta = beta;
    if beta < 0.0 {
        while beta < 0.0 {
            beta += PI;
        }
    } else {
        beta = repeat(beta, TAU);
    }

    let sin_alpha = alpha.sin();

    Vec3::new(
        sin_alpha * beta.cos() * radius,
        alpha.cos() * radius,
        sin_alpha * beta.sin() * radius,
    )
}

pub fn random_point_in_sphere(rng: &mut SmallRng, radius: f32) -> Vec3 {
    let u: f32 = rng.gen_range(0.0..1.0);
    let v: f32 = rng.gen_range(0.0..1.0);

    let theta = u * TAU;
    let phi = (2.0 * v - 1.0).acos();
    let r = rng.gen_range(0.0..radius).cbrt();

    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();

    Vec3::new(
        r * sin_phi * cos_theta,
        r * sin_phi * sin_theta,
        r * cos_phi,
    )
}

#[inline(always)]
pub fn mix_values(a: f32, b: f32, weight_b: f32) -> f32 {
    (b * weight_b) + (a * (1.0 - weight_b))
}

#[inline(always)]
pub fn repeat(value: f32, length: f32) -> f32 {
    (value - (value / length).floor() * length).clamp(0.0, length)
}

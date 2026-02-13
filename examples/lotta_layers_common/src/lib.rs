// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Shared constants and animation logic for the `lotta-layers` examples.

#![no_std]

use subduction_core::layer::{LayerId, LayerStore};
use subduction_core::transform::Transform3d;

/// Side length of each child layer square.
pub const LAYER_SIZE: f64 = 10.0;

/// Radius at which groups orbit the center.
pub const GROUP_ORBIT_RADIUS: f64 = 250.0;

/// Radius at which children orbit their group.
pub const CHILD_ORBIT_RADIUS: f64 = 80.0;

/// Converts HSL (hue 0–360, saturation 0–1, lightness 0–1) to RGB 0–1.
pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> [f64; 3] {
    let c = (1.0 - libm::fabs(2.0 * l - 1.0)) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - libm::fabs(h_prime % 2.0 - 1.0));
    let (r1, g1, b1) = if h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = l - c / 2.0;
    [r1 + m, g1 + m, b1 + m]
}

/// Animates grouped orbital layers for a single frame.
///
/// Each group orbits the center point (`cx`, `cy`) at [`GROUP_ORBIT_RADIUS`],
/// and each child orbits its group at [`CHILD_ORBIT_RADIUS`].
///
/// `t` is elapsed time in seconds.
pub fn animate_groups(
    store: &mut LayerStore,
    group_ids: &[LayerId],
    child_ids: &[LayerId],
    num_groups: usize,
    layers_per_group: usize,
    cx: f64,
    cy: f64,
    t: f64,
) {
    for (g, &group_id) in group_ids.iter().enumerate() {
        let group_phase = g as f64 * core::f64::consts::TAU / num_groups as f64;
        let group_speed = 0.3 + g as f64 * 0.02;
        let group_angle = t * group_speed + group_phase;

        let gx = cx + GROUP_ORBIT_RADIUS * libm::cos(group_angle);
        let gy = cy + GROUP_ORBIT_RADIUS * libm::sin(group_angle);

        // Group transform: translate to orbit position.
        store.set_transform(group_id, Transform3d::from_translation(gx, gy, 0.0));

        // Group opacity: pulse between 0.5–1.0.
        #[expect(
            clippy::cast_possible_truncation,
            reason = "opacity API is f32 and pulse is constrained to 0.0..=1.0"
        )]
        let pulse = (0.5 + 0.5 * libm::sin(t * 0.8 + group_phase)) as f32;
        store.set_opacity(group_id, 0.5 + pulse * 0.5);

        // Animate children within this group.
        let base = g * layers_per_group;
        for c in 0..layers_per_group {
            let child_id = child_ids[base + c];
            let child_phase = c as f64 * core::f64::consts::TAU / layers_per_group as f64;
            let child_speed = 1.5 + c as f64 * 0.01;
            let child_angle = t * child_speed + child_phase;

            let lx = CHILD_ORBIT_RADIUS * libm::cos(child_angle);
            let ly = CHILD_ORBIT_RADIUS * libm::sin(child_angle);

            // Child transform: offset from group + rotation + center offset.
            let half = LAYER_SIZE / 2.0;
            store.set_transform(
                child_id,
                Transform3d::from_translation(lx, ly, 0.0)
                    * Transform3d::from_rotation_z(t * 2.0 + child_phase)
                    * Transform3d::from_translation(-half, -half, 0.0),
            );
        }
    }
}

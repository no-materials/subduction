// Copyright 2026 the Subduction Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Minimal column-major 4×4 transform.
//!
//! This type covers the subset of 3-D affine transforms that `subduction_core`
//! actually needs (identity, multiply, column access, field mutation) without
//! pulling in a full linear-algebra crate.

use core::ops::Mul;
#[cfg(not(feature = "std"))]
use kurbo::common::FloatFuncs as _;

/// A column-major 4×4 affine transform stored as `[[f64; 4]; 4]`.
///
/// Each inner array is one *column* of the matrix, matching the memory layout
/// used by GPU APIs and Core Animation's `CATransform3D`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform3d {
    /// Four columns, each a 4-element array `[x, y, z, w]`.
    pub cols: [[f64; 4]; 4],
}

impl Transform3d {
    /// The 4×4 identity matrix.
    pub const IDENTITY: Self = Self {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    /// Creates a transform from four column arrays.
    #[inline]
    #[must_use]
    pub const fn from_cols(col0: [f64; 4], col1: [f64; 4], col2: [f64; 4], col3: [f64; 4]) -> Self {
        Self {
            cols: [col0, col1, col2, col3],
        }
    }

    /// Creates a transform from a column-major 2-D array.
    #[inline]
    #[must_use]
    pub const fn from_cols_array_2d(cols: [[f64; 4]; 4]) -> Self {
        Self { cols }
    }

    /// Returns the columns as a 2-D array.
    #[inline]
    #[must_use]
    pub const fn to_cols_array_2d(self) -> [[f64; 4]; 4] {
        self.cols
    }

    /// Returns column `i` (0-based).
    ///
    /// # Panics
    ///
    /// Panics if `i >= 4`.
    #[inline]
    #[must_use]
    pub const fn col(self, i: usize) -> [f64; 4] {
        self.cols[i]
    }

    /// Creates a pure translation transform.
    #[inline]
    #[must_use]
    pub const fn from_translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            cols: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }

    /// Creates a non-uniform scale transform.
    #[inline]
    #[must_use]
    pub const fn from_scale(sx: f64, sy: f64, sz: f64) -> Self {
        Self {
            cols: [
                [sx, 0.0, 0.0, 0.0],
                [0.0, sy, 0.0, 0.0],
                [0.0, 0.0, sz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Creates a rotation around the Z axis (radians).
    #[inline]
    #[must_use]
    pub fn from_rotation_z(radians: f64) -> Self {
        #[cfg(feature = "std")]
        let (s, c) = radians.sin_cos();
        #[cfg(not(feature = "std"))]
        let (s, c) = (radians.sin(), radians.cos());
        Self {
            cols: [
                [c, s, 0.0, 0.0],
                [-s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Is this transform [finite]?
    ///
    /// [finite]: f64::is_finite
    #[inline]
    #[must_use]
    pub const fn is_finite(&self) -> bool {
        let c = &self.cols;
        c[0][0].is_finite()
            && c[0][1].is_finite()
            && c[0][2].is_finite()
            && c[0][3].is_finite()
            && c[1][0].is_finite()
            && c[1][1].is_finite()
            && c[1][2].is_finite()
            && c[1][3].is_finite()
            && c[2][0].is_finite()
            && c[2][1].is_finite()
            && c[2][2].is_finite()
            && c[2][3].is_finite()
            && c[3][0].is_finite()
            && c[3][1].is_finite()
            && c[3][2].is_finite()
            && c[3][3].is_finite()
    }

    /// Is this transform [NaN]?
    ///
    /// [NaN]: f64::is_nan
    #[inline]
    #[must_use]
    pub const fn is_nan(&self) -> bool {
        let c = &self.cols;
        c[0][0].is_nan()
            || c[0][1].is_nan()
            || c[0][2].is_nan()
            || c[0][3].is_nan()
            || c[1][0].is_nan()
            || c[1][1].is_nan()
            || c[1][2].is_nan()
            || c[1][3].is_nan()
            || c[2][0].is_nan()
            || c[2][1].is_nan()
            || c[2][2].is_nan()
            || c[2][3].is_nan()
            || c[3][0].is_nan()
            || c[3][1].is_nan()
            || c[3][2].is_nan()
            || c[3][3].is_nan()
    }
}

impl Default for Transform3d {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Transform3d {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let a = &self.cols;
        let b = &rhs.cols;
        let mut out = [[0.0_f64; 4]; 4];
        let mut j = 0;
        while j < 4 {
            let mut i = 0;
            while i < 4 {
                out[j][i] =
                    a[0][i] * b[j][0] + a[1][i] * b[j][1] + a[2][i] * b[j][2] + a[3][i] * b[j][3];
                i += 1;
            }
            j += 1;
        }
        Self { cols: out }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_identity() {
        assert_eq!(Transform3d::default(), Transform3d::IDENTITY);
    }

    #[test]
    fn identity_multiply() {
        let t = Transform3d::from_translation(1.0, 2.0, 3.0);
        assert_eq!(Transform3d::IDENTITY * t, t);
        assert_eq!(t * Transform3d::IDENTITY, t);
    }

    #[test]
    fn translation_composition() {
        let a = Transform3d::from_translation(1.0, 0.0, 0.0);
        let b = Transform3d::from_translation(0.0, 2.0, 0.0);
        let c = a * b;
        // Combined translation should be (1, 2, 0).
        let col3 = c.col(3);
        assert_eq!(col3, [1.0, 2.0, 0.0, 1.0]);
    }

    #[test]
    fn scale() {
        let s = Transform3d::from_scale(2.0, 3.0, 4.0);
        assert_eq!(s.col(0)[0], 2.0);
        assert_eq!(s.col(1)[1], 3.0);
        assert_eq!(s.col(2)[2], 4.0);
        assert_eq!(s.col(3), [0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn round_trip_cols_array_2d() {
        let t = Transform3d::from_translation(5.0, 6.0, 7.0);
        let arr = t.to_cols_array_2d();
        assert_eq!(Transform3d::from_cols_array_2d(arr), t);
    }

    #[test]
    fn scale_then_translate() {
        let s = Transform3d::from_scale(2.0, 2.0, 2.0);
        let t = Transform3d::from_translation(3.0, 4.0, 0.0);
        // Scale first, then translate: T * S
        let combined = t * s;
        // Column 0 should be scaled.
        assert_eq!(combined.col(0), [2.0, 0.0, 0.0, 0.0]);
        // Translation column should be unchanged (translation applied after).
        assert_eq!(combined.col(3), [3.0, 4.0, 0.0, 1.0]);
    }

    #[test]
    fn rotation_z_ninety_degrees() {
        let r = Transform3d::from_rotation_z(core::f64::consts::FRAC_PI_2);
        // cos=0, sin=1 for +90deg.
        let eps = 1e-6;
        assert!((r.col(0)[0] - 0.0).abs() < eps);
        assert!((r.col(0)[1] - 1.0).abs() < eps);
        assert!((r.col(1)[0] + 1.0).abs() < eps);
        assert!((r.col(1)[1] - 0.0).abs() < eps);
    }

    #[test]
    fn identity_is_finite() {
        assert!(Transform3d::IDENTITY.is_finite());
        assert!(!Transform3d::IDENTITY.is_nan());
    }

    #[test]
    fn nan_detected() {
        let mut t = Transform3d::IDENTITY;
        t.cols[2][1] = f64::NAN;
        assert!(!t.is_finite());
        assert!(t.is_nan());
    }

    #[test]
    fn infinity_detected() {
        let mut t = Transform3d::IDENTITY;
        t.cols[0][3] = f64::INFINITY;
        assert!(!t.is_finite());
        assert!(!t.is_nan());
    }
}

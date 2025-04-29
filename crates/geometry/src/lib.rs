//! Basic 3-D geometry primitives.

#![warn(clippy::all, rust_2018_idioms)]

use nalgebra as na;

/// A point in 3-D space (Å by convention).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D(pub f64, pub f64, pub f64);

impl Point3D {
    /// Euclidean distance to another point.
    pub fn distance(self, other: Self) -> f64 {
        ((self.0 - other.0).powi(2)
            + (self.1 - other.1).powi(2)
            + (self.2 - other.2).powi(2))
        .sqrt()
    }

    /// Convert to nalgebra vector (column).
    pub fn to_vec(self) -> na::Vector3<f64> {
        na::Vector3::new(self.0, self.1, self.2)
    }
}

use std::ops::{Add, Sub};

impl Add for Point3D {
    type Output = Point3D;
    fn add(self, rhs: Point3D) -> Self::Output {
        Point3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Point3D {
    type Output = Point3D;
    fn sub(self, rhs: Point3D) -> Self::Output {
        Point3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

/// Dot product of two vectors represented by points (origin implicit).
pub fn dot(a: Point3D, b: Point3D) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

/// Length (norm) of the vector represented by the point.
pub fn norm(v: Point3D) -> f64 {
    (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn distance_symmetry() {
        let a = Point3D(0.0, 0.0, 0.0);
        let b = Point3D(1.0, 2.0, 2.0);
        assert_relative_eq!(a.distance(b), b.distance(a));
        assert_relative_eq!(a.distance(b), 3.0);
    }

    #[test]
    fn add_sub() {
        let a = Point3D(1.0, 2.0, 3.0);
        let b = Point3D(0.5, -1.0, 1.0);
        let c = a + b;
        assert_eq!(c, Point3D(1.5, 1.0, 4.0));
        let d = c - a;
        assert_eq!(d, b);
    }

    #[test]
    fn dot_and_norm() {
        let v = Point3D(3.0, 4.0, 0.0);
        assert_relative_eq!(norm(v), 5.0);
        let u = Point3D(1.0, 0.0, 0.0);
        assert_eq!(dot(v, u), 3.0);
    }
}

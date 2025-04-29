//! Basic 3-D geometry primitives.

#![warn(clippy::all, rust_2018_idioms)]

use nalgebra as na;

/// A point in 3-D space (Å by convention).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D(pub f64, pub f64, pub f64);

/// A 3-D vector (direction + magnitude).  Thin newtype around `nalgebra::Vector3`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D(pub na::Vector3<f64>);

impl Vector3D {
    /// Construct from raw components.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(na::Vector3::new(x, y, z))
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }
    pub fn y(&self) -> f64 {
        self.0.y
    }
    pub fn z(&self) -> f64 {
        self.0.z
    }

    /// Dot product.
    pub fn dot(self, other: Self) -> f64 {
        self.0.dot(&other.0)
    }

    /// Cross product.
    pub fn cross(self, other: Self) -> Self {
        Self(self.0.cross(&other.0))
    }

    /// Euclidean norm (length).
    pub fn norm(self) -> f64 {
        self.0.norm()
    }

    /// Return a normalised vector (unit length).  Returns `None` for zero vector.
    pub fn normalized(self) -> Option<Self> {
        let n = self.norm();
        if n.abs() < f64::EPSILON {
            None
        } else {
            Some(Self(self.0 / n))
        }
    }
}

// --- Operators -------------------------------------------------------------

use std::ops::{Add, Sub, Mul, Neg, Div};

impl Add for Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Vector3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Vector3D {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for Vector3D {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl Neg for Vector3D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

// ---------------------------------------------------------------------------
// Quaternion & rotations -----------------------------------------------------

pub type Quaternion = na::UnitQuaternion<f64>;

/// Rotation utilities.
pub mod rotation {
    use super::{Quaternion, Vector3D, na};

    /// Construct a quaternion representing rotation of `angle_rad` around given axis.
    pub fn axis_angle(axis: Vector3D, angle_rad: f64) -> Quaternion {
        Quaternion::from_axis_angle(&na::Unit::new_normalize(axis.0), angle_rad)
    }

    /// Rotate a vector by a quaternion.
    pub fn rotate_vec(q: &Quaternion, v: Vector3D) -> Vector3D {
        Vector3D(q.transform_vector(&v.0))
    }
}

// ---------------------------------------------------------------------------
// Affine transform (4×4 matrix) ---------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform3D(pub na::Matrix4<f64>);

impl Transform3D {
    /// Identity transform.
    pub fn identity() -> Self {
        Self(na::Matrix4::identity())
    }

    /// Translation by vector.
    pub fn from_translation(v: Vector3D) -> Self {
        let mut m = na::Matrix4::identity();
        m[(0, 3)] = v.x();
        m[(1, 3)] = v.y();
        m[(2, 3)] = v.z();
        Self(m)
    }

    /// Rotation from a quaternion (no translation).
    pub fn from_quaternion(q: Quaternion) -> Self {
        Self(q.to_homogeneous())
    }

    /// Combine two transforms (self · other).
    pub fn then(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

    /// Apply transform to a point.
    pub fn apply_point(self, p: Point3D) -> Point3D {
        let v = na::Vector4::new(p.0, p.1, p.2, 1.0);
        let res = self.0 * v;
        Point3D(res.x, res.y, res.z)
    }
}

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



// Note: arithmetic between two points is undefined in strict geometry terms.
// We therefore *do not* implement `Add<Point3D>`; subtraction yields a vector
// via the blanket impl below.

// Produce a Vector3D from point subtraction --------------------------------

impl std::ops::Sub<Point3D> for Point3D {
    type Output = Vector3D;
    fn sub(self, rhs: Point3D) -> Self::Output {
        Vector3D::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// Translate a point by a vector --------------------------------------------

impl std::ops::Add<Vector3D> for Point3D {
    type Output = Point3D;
    fn add(self, rhs: Vector3D) -> Self::Output {
        Point3D(self.0 + rhs.x(), self.1 + rhs.y(), self.2 + rhs.z())
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

/// Dot product of two Vector3D.
pub fn dot_v(a: Vector3D, b: Vector3D) -> f64 {
    a.dot(b)
}

/// Norm of Vector3D.
pub fn norm_v(v: Vector3D) -> f64 {
    v.norm()
}

// ---------------------------------------------------------------------------
// Extended tests ------------------------------------------------------------

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
    fn translate_point() {
        let p = Point3D(1.0, 2.0, 3.0);
        let v = Vector3D::new(0.5, -1.0, 1.0);
        let p2 = p + v;
        assert_eq!(p2, Point3D(1.5, 1.0, 4.0));
        let back = p2 - p;
        assert_eq!(back, v);
    }

    #[test]
    fn dot_and_norm() {
        let v = Point3D(3.0, 4.0, 0.0);
        assert_relative_eq!(norm(v), 5.0);
        let u = Point3D(1.0, 0.0, 0.0);
        assert_eq!(dot(v, u), 3.0);
    }

    #[test]
    fn point_vector_ops() {
        let p1 = Point3D(1.0, 2.0, 3.0);
        let p2 = Point3D(2.0, 0.0, 1.0);
        let v = p1 - p2;
        assert_eq!(v, Vector3D::new(-1.0, 2.0, 2.0));

        let p3 = p2 + v;
        assert_eq!(p3, p1);
    }

    #[test]
    fn vector3d_ops() {
        let a = Vector3D::new(1.0, 0.0, 0.0);
        let b = Vector3D::new(0.0, 1.0, 0.0);
        let c = a + b;
        assert_relative_eq!(c.norm(), (2.0f64).sqrt());
        let cross = a.cross(b);
        assert_eq!(cross, Vector3D::new(0.0, 0.0, 1.0));
        let dot = a.dot(b);
        assert_eq!(dot, 0.0);
    }

    #[test]
    fn quaternion_rotation() {
        // 90° around Z axis should rotate (1,0,0) -> (0,1,0)
        let axis = Vector3D::new(0.0, 0.0, 1.0);
        let q = rotation::axis_angle(axis, std::f64::consts::FRAC_PI_2);
        let v = Vector3D::new(1.0, 0.0, 0.0);
        let rotated = rotation::rotate_vec(&q, v);
        assert_relative_eq!(rotated.x(), 0.0, epsilon = 1e-12);
        assert_relative_eq!(rotated.y(), 1.0, epsilon = 1e-12);
        assert_relative_eq!(rotated.z(), 0.0, epsilon = 1e-12);
    }

    #[test]
    fn transform_apply() {
        let translation = Vector3D::new(1.0, 2.0, 3.0);
        let t = Transform3D::from_translation(translation);
        let p = Point3D(0.0, 0.0, 0.0);
        let res = t.apply_point(p);
        assert_eq!(res, Point3D(1.0, 2.0, 3.0));
    }
}

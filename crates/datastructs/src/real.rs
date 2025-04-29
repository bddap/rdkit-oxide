//! RealValueVect – sparse vector mapping indices to `f64` values.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct RealValueVect {
    size: u32,
    data: BTreeMap<u32, f64>,
}

impl RealValueVect {
    pub fn new(size: u32) -> Self {
        Self {
            size,
            data: BTreeMap::new(),
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn set_val(&mut self, idx: u32, val: f64) {
        assert!(idx < self.size, "index out of range");
        if val.abs() < f64::EPSILON {
            self.data.remove(&idx);
        } else {
            self.data.insert(idx, val);
        }
    }

    pub fn get_val(&self, idx: u32) -> f64 {
        assert!(idx < self.size, "index out of range");
        *self.data.get(&idx).unwrap_or(&0.0)
    }

    pub fn sum_values(&self) -> f64 {
        self.data.values().sum()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        assert_eq!(self.size, other.size);
        let (smaller, bigger) = if self.data.len() <= other.data.len() {
            (self, other)
        } else {
            (other, self)
        };
        smaller
            .data
            .iter()
            .map(|(idx, val)| *val * bigger.get_val(*idx))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn set_get() {
        let mut v = RealValueVect::new(10);
        v.set_val(3, 1.5);
        v.set_val(8, -2.0);
        assert_relative_eq!(v.get_val(3), 1.5);
        assert_relative_eq!(v.get_val(8), -2.0);
        assert_eq!(v.get_val(0), 0.0);
    }

    #[test]
    fn dot_product() {
        let mut a = RealValueVect::new(5);
        a.set_val(0, 1.0);
        a.set_val(2, 2.0);
        let mut b = RealValueVect::new(5);
        b.set_val(0, -1.0);
        b.set_val(2, 3.0);
        assert_relative_eq!(a.dot(&b), -1.0 + 6.0);
    }
}

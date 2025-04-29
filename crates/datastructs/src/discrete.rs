//! DiscreteValueVect – sparse vector mapping indices to `i32` counts.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DiscreteValueVect {
    size: u32,
    data: BTreeMap<u32, i32>,
}

impl DiscreteValueVect {
    pub fn new(size: u32) -> Self {
        Self {
            size,
            data: BTreeMap::new(),
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn set_val(&mut self, idx: u32, val: i32) {
        assert!(idx < self.size, "index out of range");
        if val == 0 {
            self.data.remove(&idx);
        } else {
            self.data.insert(idx, val);
        }
    }

    pub fn get_val(&self, idx: u32) -> i32 {
        assert!(idx < self.size, "index out of range");
        *self.data.get(&idx).unwrap_or(&0)
    }

    pub fn dot(&self, other: &Self) -> i32 {
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

    #[test]
    fn set_get() {
        let mut v = DiscreteValueVect::new(8);
        v.set_val(3, 5);
        assert_eq!(v.get_val(3), 5);
        v.set_val(3, 0);
        assert_eq!(v.get_val(3), 0);
    }

    #[test]
    fn dot_product() {
        let mut a = DiscreteValueVect::new(4);
        a.set_val(1, 2);
        a.set_val(2, 3);
        let mut b = DiscreteValueVect::new(4);
        b.set_val(1, 4);
        b.set_val(3, 5);
        assert_eq!(a.dot(&b), 8);
    }
}

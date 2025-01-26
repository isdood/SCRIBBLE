//! Iterator implementations for crystal arrays

use super::CrystalArray;
use std::iter::FromIterator;

impl<T: Clone> FromIterator<T> for CrystalArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (min, _) = iter.size_hint();

        let mut array = Self::with_capacity(
            min,
            Self::optimal_alignment()
        );

        for item in iter {
            array.push(item);
        }

        array
    }
}

impl<T: Clone> IntoIterator for CrystalArray<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            array: self,
            index: 0,
        }
    }
}

pub struct IntoIter<T> {
    array: CrystalArray<T>,
    index: usize,
}

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            let value = self.array.get(self.index).map(|v| v.clone());
            self.index += 1;
            value
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.array.len() - self.index;
        (remaining, Some(remaining))
    }
}

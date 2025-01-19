// lib/quartz/src/vecdek.rs

//! VecDek - Crystal-based Double-Ended Queue
//! Last Updated: 2025-01-19 16:20:32 UTC
//! Author: isdood

pub struct VecDek<T> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
    capacity: usize,
}

impl<T> VecDek<T> {
    /// Create a new VecDek with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
            capacity,
        }
    }

    /// Push an element to the front of the VecDek
    pub fn push_front(&mut self, value: T) {
        if self.len() == self.capacity {
            self.expand_capacity();
        }
        self.head = (self.head + self.capacity - 1) % self.capacity;
        if self.buffer.len() < self.capacity {
            self.buffer.insert(self.head, value);
        } else {
            self.buffer[self.head] = value;
        }
    }

    /// Push an element to the back of the VecDek
    pub fn push_back(&mut self, value: T) {
        if self.len() == self.capacity {
            self.expand_capacity();
        }
        if self.buffer.len() < self.capacity {
            self.buffer.push(value);
        } else {
            self.buffer[self.tail] = value;
        }
        self.tail = (self.tail + 1) % self.capacity;
    }

    /// Pop an element from the front of the VecDek
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = std::mem::replace(&mut self.buffer[self.head], unsafe { std::mem::zeroed() });
        self.head = (self.head + 1) % self.capacity;
        Some(value)
    }

    /// Pop an element from the back of the VecDek
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        let value = std::mem::replace(&mut self.buffer[self.tail], unsafe { std::mem::zeroed() });
        Some(value)
    }

    /// Get the current number of elements in the VecDek
    pub fn len(&self) -> usize {
        if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.capacity - self.head + self.tail
        }
    }

    /// Check if the VecDek is empty
    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    /// Expand the capacity of the VecDek
    fn expand_capacity(&mut self) {
        let new_capacity = self.capacity * 2;
        let mut new_buffer = Vec::with_capacity(new_capacity);

        for i in 0..self.len() {
            let index = (self.head + i) % self.capacity;
            new_buffer.push(std::mem::replace(&mut self.buffer[index], unsafe { std::mem::zeroed() }));
        }

        self.buffer = new_buffer;
        self.head = 0;
        self.tail = self.capacity;
        self.capacity = new_capacity;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecdek_creation() {
        let dek: VecDek<i32> = VecDek::with_capacity(4);
        assert!(dek.is_empty());
    }

    #[test]
    fn test_push_pop_front() {
        let mut dek = VecDek::with_capacity(4);
        dek.push_front(1);
        dek.push_front(2);
        assert_eq!(dek.pop_front(), Some(2));
        assert_eq!(dek.pop_front(), Some(1));
    }

    #[test]
    fn test_push_pop_back() {
        let mut dek = VecDek::with_capacity(4);
        dek.push_back(1);
        dek.push_back(2);
        assert_eq!(dek.pop_back(), Some(2));
        assert_eq!(dek.pop_back(), Some(1));
    }

    #[test]
    fn test_push_front_expand() {
        let mut dek = VecDek::with_capacity(2);
        dek.push_front(1);
        dek.push_front(2);
        dek.push_front(3);
        assert_eq!(dek.len(), 3);
    }

    #[test]
    fn test_push_back_expand() {
        let mut dek = VecDek::with_capacity(2);
        dek.push_back(1);
        dek.push_back(2);
        dek.push_back(3);
        assert_eq!(dek.len(), 3);
    }
}

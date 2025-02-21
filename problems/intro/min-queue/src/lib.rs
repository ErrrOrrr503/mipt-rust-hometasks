#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    data: VecDeque<T>,
    history: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            history: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: T) {
        match self.history.back() {
            Some(mut m) if &val < m => {
                while &val < m {
                    self.history.pop_back();
                    if self.history.len() == 0 {
                        break;
                    }
                    m = self.history.back().unwrap();
                }
            },
            _ => {},
        }
        self.history.push_back(val.clone());
        self.data.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.data.pop_front() {
            Some(val) => {
                match self.min() {
                    Some(min) if min == &val => {
                        self.history.pop_front();
                        Some(val)
                    },
                    _ => Some(val),
                }
            },
            None => None,
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.data.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.history.front()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

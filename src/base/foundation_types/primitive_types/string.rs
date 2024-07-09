use std::{fmt, hash};
use std::fmt::Debug;
use std::ops::Add;

use crate::base::foundation_types::primitive_types::any::Any;
use crate::base::foundation_types::primitive_types::ordered::Ordered;

pub struct String {
    pub value: std::string::String
}

impl String {
    pub fn new(value: std::string::String) -> Self {
        Self { value }
    }
    pub fn is_integer(&self) -> bool {
        self.value.parse::<i32>().is_ok()
    }
    pub fn as_integer(&self) -> Result<i32, std::num::ParseIntError> {
        self.value.parse::<i32>()
    }
    pub  fn into_bytes(self) -> Vec<u8> {
        self.value.into_bytes()
    }
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
    pub fn as_mut_str(&mut self) -> &mut str {
        self.value.as_mut_str()
    }
    pub fn push_str(&mut self, string: &str) {
        self.value.push_str(string)
    }
    pub fn push(&mut self, c: char) {
        self.value.push(c)
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }
    pub fn truncate(&mut self, new_len: usize) {
        self.value.truncate(new_len)
    }
    pub fn pop(&mut self) -> Option<char> {
        self.value.pop()
    }
    pub fn remove(&mut self, index: usize) -> char {
        self.value.remove(index)
    }
    pub fn retain<F>(&mut self, mut f: F) where F: FnMut(char) -> bool {
        self.value.retain(|c| f(c))
    }
    pub fn insert(&mut self, index: usize, c: char) {
        self.value.insert(index, c)
    }
    pub fn insert_str(&mut self, index: usize, string: &str) {
        self.value.insert_str(index, string)
    }
    pub fn len(&self) -> usize {
        self.value.len()
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn split_off(&mut self, at: usize) -> Self {
        Self::new(self.value.split_off(at))
    }
    pub fn clear(&mut self) {
        self.value.clear()
    }
    pub fn replace_range<R>(&mut self, range: R, replace_with: &str) where R: std::ops::RangeBounds<usize> {
        self.value.replace_range(range, replace_with)
    }
    pub fn into_boxed_str(self) -> Box<str> {
        self.value.into_boxed_str()
    }

}

impl Add<&str> for String {
    type Output = Self;

    fn add(mut self, other: &str) -> Self::Output {
        self.value.push_str(other);
        self
    }

}

impl Add<&String> for String {
    type Output = Self;

    fn add(mut self, other: &Self) -> Self::Output {
        self.value.push_str(other.value.as_str());
        self
    }

}


impl Any for String {
    fn is_equal(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<String>() {
            self == other
        } else {
            false
        }
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for String {}

impl Ordered for String {}

impl PartialOrd for String {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl hash::Hash for String {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}




use std::collections::HashMap;

pub struct Counter {
    map: HashMap<usize, usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn add(&mut self, count: usize) {
        *self.map.entry(count).or_insert(0) += 1;
    }
    pub fn get(&mut self, count: usize) -> Option<usize> {
        self.map.get(&count).map(|v| *v)
    }
    pub fn sum(&self) -> usize {
        self.map.iter().map(|(k, v)| k * v).sum()
    }
    pub fn count(&self) -> usize {
        self.map.values().sum()
    }
    pub fn max(&self) -> usize {
        *self.map.keys().max().unwrap()
    }
    pub fn as_vec(&self) -> Vec<(usize, usize)> {
        let mut vec: Vec<(usize, usize)> = self.map.iter().map(|(a, b)| (*a, *b)).collect();
        vec.sort_by_key(|t| t.0);
        vec
    }
}

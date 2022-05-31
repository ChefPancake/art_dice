use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
pub struct ItemCounter<T: Hash + PartialEq + Eq + PartialOrd + Ord + Clone> {
    items: HashMap<T, usize>
}

impl<T: Hash + PartialEq + Eq + PartialOrd + Ord + Clone> Hash for ItemCounter<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut items: Vec<T> = self.items.keys().cloned().collect();
        items.sort();
        for item in items {
            for _ in 0..(self.items[&item]) {
                item.hash(state);
            }
        }
    }
}

impl<T: Hash + PartialEq + Eq + PartialOrd + Ord + Clone> ItemCounter<T> {
    pub fn new() -> ItemCounter<T> {
        ItemCounter {
            items: HashMap::new()
        }
    }

    pub fn add(&mut self, item: &T) {
        self.add_amount(item, 1)
    }

    pub fn add_amount(&mut self, item: &T, amount: usize) {
        if self.items.contains_key(item) {
            self.items.get_mut(item).map(|x| *x += amount);
        } else {
            self.items.insert(item.clone(), amount);
        }
    }

    pub fn get_count(&self, item: &T) -> usize {
        *self.items.get(item).unwrap_or(&0)
    }

    pub fn total_count(&self) -> usize {
        self.items.values().sum()
    }
}
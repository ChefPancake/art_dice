use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Eq, Clone)]
/// A struct for counting occurrences of an object
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
    /// Creates a new, empty instance of [`ItemCounter`](crate::item_counter::ItemCounter)
    /// 
    /// # Example
    /// ```rust 
    /// # use art_dice::item_counter::ItemCounter;
    /// # fn main() -> Result<(), String> {
    /// let counter = ItemCounter::<i32>::new();
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> ItemCounter<T> {
        ItemCounter {
            items: HashMap::new()
        }
    }

    /// Adds an item to the counter, incrementing its internal count by one
    /// 
    /// # Example
    /// ```rust 
    /// # use art_dice::item_counter::ItemCounter;
    /// # fn main() -> Result<(), String> {
    /// let mut counter = ItemCounter::<i32>::new();
    /// 
    /// counter.add(&5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add(&mut self, item: &T) {
        self.add_amount(item, 1)
    }

    /// Adds an item to the counter, incrementing its internal count by amount
    /// 
    /// # Example
    /// ```rust 
    /// # use art_dice::item_counter::ItemCounter;
    /// # fn main() -> Result<(), String> {
    /// let mut counter = ItemCounter::<i32>::new();
    /// 
    /// counter.add_amount(&7, 4);
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_amount(&mut self, item: &T, amount: usize) {
        if self.items.contains_key(item) {
            self.items.get_mut(item).map(|x| *x += amount);
        } else {
            self.items.insert(item.clone(), amount);
        }
    }

    /// Retreives the internal count of the provided item. If the item was never added, returns 0
    /// 
    /// # Example
    /// ```rust 
    /// # use art_dice::item_counter::ItemCounter;
    /// # fn main() -> Result<(), String> {
    /// # let mut counter = ItemCounter::<i32>::new();
    /// counter.add(&8);
    /// counter.add_amount(&3, 2);
    /// 
    /// assert_eq!(counter.get_count(&8), 1);
    /// assert_eq!(counter.get_count(&3), 2);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_count(&self, item: &T) -> usize {
        match self.items.get(item) {
            Some(x) => *x,
            None => 0
        }
    }
}
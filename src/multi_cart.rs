pub struct MultiCartesianProduct<'a, T: 'a + Clone> {
    sets: &'a [&'a [T]],
    maximums: Vec<usize>,
    indexes: Vec<usize>,
    is_complete: bool
}

impl<'a, T: Clone> MultiCartesianProduct<'a, T> {
    pub fn new(sets: &'a [&'a [T]]) -> MultiCartesianProduct<'a, T> {
        let set_count = sets.len();
        let mut maximums = Vec::with_capacity(set_count);
        let mut indexes = Vec::with_capacity(set_count);
        for index in 0..set_count {
            maximums.insert(index, sets[index].len());
            indexes.insert(index, 0);
        }
        MultiCartesianProduct {
            sets,
            maximums,
            indexes,
            is_complete: false
        }
    }

    fn increment_counter(&mut self) {
        let mut incrementing = true;
        let mut index = 0;
        let index_len = self.indexes.len();
        while incrementing && (index < index_len) {
            self.indexes[index] = (self.indexes[index] + 1) % self.maximums[index];
            incrementing = self.indexes[index] == 0;
            index += 1;
        }
        self.is_complete =
            (index == index_len) 
            && (incrementing == true);
    }
}

impl<'a, T: Clone> Iterator for MultiCartesianProduct<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.is_complete {
            return None;
        }
        let to_return =
            self.sets.iter()
            .enumerate()
            .map(|(i, &x)| x[self.indexes[i]].clone())
            .collect();
        self.increment_counter();
        Some(to_return)
    }
}




#[cfg(test)]
mod tests {
    use crate::multi_cart::MultiCartesianProduct;

    #[test]
    fn two_sets() {
        let set1 = vec![1, 2, 3];
        let set2 = vec![4, 5];
        let all_sets = vec![ set1.as_slice(), set2.as_slice() ];
        let mut cart = MultiCartesianProduct::new(&all_sets);
        
        assert_eq!(cart.next(), Some(vec![1,4]));
        assert_eq!(cart.next(), Some(vec![2,4]));
        assert_eq!(cart.next(), Some(vec![3,4]));

        assert_eq!(cart.next(), Some(vec![1,5]));
        assert_eq!(cart.next(), Some(vec![2,5]));
        assert_eq!(cart.next(), Some(vec![3,5]));

        assert_eq!(cart.next(), None);
    }

    #[test]
    fn three_sets() {
        let set1 = vec![1, 2, 3];
        let set2 = vec![4, 5];
        let set3 = vec![6, 7, 8];
        let all_sets = vec![ set1.as_slice(), set2.as_slice(), set3.as_slice() ];
        let mut cart = MultiCartesianProduct::new(&all_sets);
        
        assert_eq!(cart.next(), Some(vec![1,4,6]));
        assert_eq!(cart.next(), Some(vec![2,4,6]));
        assert_eq!(cart.next(), Some(vec![3,4,6]));
        assert_eq!(cart.next(), Some(vec![1,5,6]));
        assert_eq!(cart.next(), Some(vec![2,5,6]));
        assert_eq!(cart.next(), Some(vec![3,5,6]));

        assert_eq!(cart.next(), Some(vec![1,4,7]));
        assert_eq!(cart.next(), Some(vec![2,4,7]));
        assert_eq!(cart.next(), Some(vec![3,4,7]));
        assert_eq!(cart.next(), Some(vec![1,5,7]));
        assert_eq!(cart.next(), Some(vec![2,5,7]));
        assert_eq!(cart.next(), Some(vec![3,5,7]));

        assert_eq!(cart.next(), Some(vec![1,4,8]));
        assert_eq!(cart.next(), Some(vec![2,4,8]));
        assert_eq!(cart.next(), Some(vec![3,4,8]));
        assert_eq!(cart.next(), Some(vec![1,5,8]));
        assert_eq!(cart.next(), Some(vec![2,5,8]));
        assert_eq!(cart.next(), Some(vec![3,5,8]));

        assert_eq!(cart.next(), None);
    }

    #[test]
    fn four_sets() {
        let set1 = vec![1, 2, 3];
        let set2 = vec![4, 5];
        let set3 = vec![6, 7, 8];
        let set4 = vec![9, 10];
        let all_sets = vec![ set1.as_slice(), set2.as_slice(), set3.as_slice(), set4.as_slice() ];
        let mut cart = MultiCartesianProduct::new(&all_sets);
        
        assert_eq!(cart.next(), Some(vec![1,4,6,9]));
        assert_eq!(cart.next(), Some(vec![2,4,6,9]));
        assert_eq!(cart.next(), Some(vec![3,4,6,9]));
        assert_eq!(cart.next(), Some(vec![1,5,6,9]));
        assert_eq!(cart.next(), Some(vec![2,5,6,9]));
        assert_eq!(cart.next(), Some(vec![3,5,6,9]));
        assert_eq!(cart.next(), Some(vec![1,4,7,9]));
        assert_eq!(cart.next(), Some(vec![2,4,7,9]));
        assert_eq!(cart.next(), Some(vec![3,4,7,9]));
        assert_eq!(cart.next(), Some(vec![1,5,7,9]));
        assert_eq!(cart.next(), Some(vec![2,5,7,9]));
        assert_eq!(cart.next(), Some(vec![3,5,7,9]));
        assert_eq!(cart.next(), Some(vec![1,4,8,9]));
        assert_eq!(cart.next(), Some(vec![2,4,8,9]));
        assert_eq!(cart.next(), Some(vec![3,4,8,9]));
        assert_eq!(cart.next(), Some(vec![1,5,8,9]));
        assert_eq!(cart.next(), Some(vec![2,5,8,9]));
        assert_eq!(cart.next(), Some(vec![3,5,8,9]));

        assert_eq!(cart.next(), Some(vec![1,4,6,10]));
        assert_eq!(cart.next(), Some(vec![2,4,6,10]));
        assert_eq!(cart.next(), Some(vec![3,4,6,10]));
        assert_eq!(cart.next(), Some(vec![1,5,6,10]));
        assert_eq!(cart.next(), Some(vec![2,5,6,10]));
        assert_eq!(cart.next(), Some(vec![3,5,6,10]));
        assert_eq!(cart.next(), Some(vec![1,4,7,10]));
        assert_eq!(cart.next(), Some(vec![2,4,7,10]));
        assert_eq!(cart.next(), Some(vec![3,4,7,10]));
        assert_eq!(cart.next(), Some(vec![1,5,7,10]));
        assert_eq!(cart.next(), Some(vec![2,5,7,10]));
        assert_eq!(cart.next(), Some(vec![3,5,7,10]));
        assert_eq!(cart.next(), Some(vec![1,4,8,10]));
        assert_eq!(cart.next(), Some(vec![2,4,8,10]));
        assert_eq!(cart.next(), Some(vec![3,4,8,10]));
        assert_eq!(cart.next(), Some(vec![1,5,8,10]));
        assert_eq!(cart.next(), Some(vec![2,5,8,10]));
        assert_eq!(cart.next(), Some(vec![3,5,8,10]));

        assert_eq!(cart.next(), None);
    }
}

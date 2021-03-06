/*
This code is part of the WhiteboxTools geospatial analysis library.
Authors: Dr. John Lindsay
Last Modified: 13/12/2018
License: MIT
*/

/// NMaximizer is can be used to find the 'n' largest values in a set of values of type T.
/// It is useful as an alternative to an approach that would use a priority queue, which
/// would have larger memory requirements (to create the queue). NMaximizer is an
/// efficient and small memory solution.
///
/// ## Example
///     let mut highs = NMaximizer::new(4);
///
///     let data = vec![4.0, 3.0, -2.0, 9.0, 3.0, 2.0, 1.0, 8.0, 5.0];
///     for val in data {
///         highs.insert(val);
///     }
///     
///     for i in 0..4 {
///         println!("{}", highs.get(i).unwrap());
///     }
pub struct NMaximizer<T: Copy + PartialOrd + PartialEq> {
    values: Vec<T>,
    n: usize,
}

impl<T: Copy + PartialOrd + PartialEq> NMaximizer<T> {
    /// Creates a new NMaximizer object
    pub fn new(n: usize) -> NMaximizer<T> {
        if n == 0 {
            panic!("Invalid NMaximizer 'n' value.");
        }
        // values must have a capacity of n+1 so that
        // there is no reallocation of the Vec after
        // insertion and before the end is popped.
        NMaximizer {
            n: n,
            values: Vec::with_capacity(n + 1),
        }
    }

    /// Inserts a value into the minimizer
    pub fn insert(&mut self, value: T) {
        if self.values.len() == self.n {
            // First see if it is less than the current smallest value in the
            // list of values. If the set is large compared to N, the probability
            // that a new value is one of the current minima is low. In this
            // way, the majority of values will not need to be sorted at all.
            // Instead, they only need to be compared with the smallest current
            // maxima.
            if value > self.values[self.n - 1] {
                for a in 0..self.n {
                    if value > self.values[a] {
                        self.values.insert(a, value);
                        self.values.pop();
                        break;
                    }
                }
            }
        } else {
            // If the size of the minima set is
            for a in 0..self.n {
                if self.values.len() == a || value > self.values[a] {
                    self.values.insert(a, value);
                    break;
                }
            }
        }
    }

    /// Inserts a vector of value into the minimizer
    pub fn insert_values<'a>(&mut self, values: &'a Vec<T>) {
        for v in values {
            self.insert(*v);
        }
    }

    /// Returns the *i*th minimum, where *i* < *n*.
    pub fn get(&self, i: usize) -> Option<T> {
        if i < self.size() {
            return Some(self.values[i]);
        }
        None
    }

    /// Returns all of the minima as a vector.
    pub fn get_maxima(&self) -> Vec<T> {
        self.values.clone()
    }

    /// Until *n* values have been inserted, the current minima
    /// set size < *n*. After *n* values have been inserted size = *n*.
    pub fn size(&self) -> usize {
        self.values.len()
    }

    /// Returns the target number of values to minimize
    /// from the inserted set.
    pub fn n(&self) -> usize {
        self.n
    }

    /// Returns true if the NMinimizer has no current values.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::NMaximizer;
    use std::cmp::Ordering;

    #[test]
    #[should_panic]
    fn test_nmaximizer_new() {
        let mut highs = NMaximizer::new(0);
        highs.insert(6.0);
    }

    #[test]
    fn test_nmaximizer_insert() {
        let mut highs = NMaximizer::new(4);

        for val in [4.0, 3.0, -2.0, 9.0, 3.0, 2.0, 1.0, 8.0].into_iter() {
            highs.insert(*val);
        }
        assert_eq!(highs.get_maxima(), vec![9.0, 8.0, 4.0, 3.0]);
    }

    #[test]
    fn test_nmaximizer_insert_values() {
        let mut highs = NMaximizer::new(4);

        let mut data = vec![4.0, 3.0, -2.0, 9.0, 3.0, 2.0, 1.0, 8.0, 5.0];
        highs.insert_values(&data);
        data.push(6.0);

        assert_eq!(highs.get_maxima(), vec![9.0, 8.0, 5.0, 4.0]);
    }

    #[test]
    fn test_nmaximizer_size() {
        let mut highs = NMaximizer::new(4);

        let data = vec![4.0, 3.0];
        highs.insert_values(&data);

        assert_eq!(highs.size(), 2);
    }

    #[test]
    fn test_nmaximizer_custom_struct() {
        let mut highs = NMaximizer::new(3);
        highs.insert(CustomStruct {
            value: 7i32,
            dist: 5f64,
        });
        highs.insert(CustomStruct {
            value: 5i32,
            dist: 13f64,
        });
        highs.insert(CustomStruct {
            value: 4i32,
            dist: 3f64,
        });
        highs.insert(CustomStruct {
            value: 6i32,
            dist: 1f64,
        });
        highs.insert(CustomStruct {
            value: 3i32,
            dist: 20f64,
        });

        assert_eq!(
            highs.get_maxima(),
            vec![
                CustomStruct {
                    value: 3i32,
                    dist: 20f64,
                },
                CustomStruct {
                    value: 5i32,
                    dist: 13f64,
                },
                CustomStruct {
                    value: 7i32,
                    dist: 5f64,
                },
            ]
        );
    }

    #[derive(Clone, Copy, Debug)]
    struct CustomStruct<T> {
        value: T,
        dist: f64,
    }

    impl<T> PartialEq for CustomStruct<T> {
        fn eq(&self, other: &CustomStruct<T>) -> bool {
            self.dist == other.dist
        }
    }

    impl<T> Eq for CustomStruct<T> {}

    impl<T> PartialOrd for CustomStruct<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.dist.partial_cmp(&other.dist)
        }
    }

}

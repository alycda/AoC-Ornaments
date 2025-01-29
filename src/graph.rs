//! Graph
//!
//! A HashMap would provide O(1) lookups vs O(log n).

// Canonical ordering: always store with lesser point first
// pub type Distances<T> = BTreeMap<(String, String), T>;

use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct Distances<T>(BTreeMap<(String, String), T>);

impl<T: std::ops::Add<Output = T> + Clone + Copy + Ord + Default> Distances<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert_ordered(&mut self, a: String, b: String, distance: T) {
        let key = if a < b { (a, b) } else { (b, a) };
        self.0.insert(key, distance);
    }

    pub fn get_distance(&self, a: &str, b: &str) -> Option<&T> {
        let key = if a < b {
            (a.to_string(), b.to_string())
        } else {
            (b.to_string(), a.to_string())
        };
        self.get(&key)
    }

    // /// not sure if btree vs hashset is better here
    // pub fn to_unique(&self) -> Uniqued {
    //     let mut set = BTreeSet::new();
    //     for (a, b) in self.keys() {
    //         set.insert(a.clone());
    //         set.insert(b.clone());
    //     }

    //     set
    // }

    /// straight line
    pub fn find_shortest_path(&self) -> Option<T> {
        TravelingSales::best(self, T::min)
    }

    /// straight line
    pub fn find_longest_path(&self) -> Option<T> {
        TravelingSales::best(self, T::max)
    }

    pub fn find_longest_circular_path(&self) -> Option<T> {
        // TravelingSales::best_circular(self, T::max)

        todo!()
    }

    pub fn get_unique(&self) -> HashSet<&str> {
        let mut unique = HashSet::new();
        for ((a, b), _) in &self.0 {
            unique.insert(a.as_str());
            unique.insert(b.as_str());
        }
        unique
    }
}

#[derive(Debug)]
pub struct TravelingSales<T>(Option<T>, fn(T, T) -> T);

impl<T> std::ops::Deref for TravelingSales<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::ops::Add<Output = T> + Clone + Copy + Ord + Default> TravelingSales<T> {
    fn new(compare: fn(T, T) -> T) -> Self {
        Self(None, compare)
    }

    /// straight line
    pub fn best(map: &Distances<T>, strategy: fn(T, T) -> T) -> Option<T> {
        let set = map.get_unique();
        let mut best_path = None;

        for start in set.iter() {
            let mut remaining = set.clone();
            remaining.remove(start); // Remove starting city from remaining set

            let mut tsp = Self::new(strategy);
            tsp.path(map, start, &mut remaining, T::default());

            // Update overall best if this path is STRATEGY
            if let Some(path_length) = *tsp {
                best_path = match best_path {
                    None => Some(path_length),
                    Some(current_best) => Some(strategy(current_best, path_length)),
                };
            }
        }

        best_path
    }

    /// straight line
    fn path(
        &mut self,
        locations: &Distances<T>,
        current: &str,
        remaining: &mut HashSet<&str>,
        running_total: T,
    ) {
        if remaining.is_empty() {
            self.0 = match self.0 {
                None => Some(running_total),
                Some(current_min) => Some((self.1)(current_min, running_total)),
            };
            return;
        }

        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };

            if let Some(distance) = locations.get(&key) {
                // dbg!(running_total, distance);

                remaining.remove(next);
                self.path(locations, next, remaining, running_total + *distance);
                remaining.insert(next);
            }
        }
    }

    fn _best_circular() {
        todo!()
    }

    fn _circular_path(
        _locations: &Distances<T>,
        _current: &str,
        remaining: &mut HashSet<&str>,
        _running_total: T,
    ) {
        if remaining.is_empty() {
            // Add the final connection back to start

            return;
        }
        let _neighbors: Vec<_> = remaining.iter().copied().collect();

        todo!()
    }
}

pub type Uniqued = BTreeSet<String>;

//! A HashMap would provide O(1) lookups vs O(log n).

// Canonical ordering: always store with lesser point first
// pub type Distances<T> = BTreeMap<(String, String), T>;

use std::collections::{BTreeMap, BTreeSet, HashSet};

#[derive(Debug)]
pub struct Distances<T>(BTreeMap<(String, String), T>);

impl<T> std::ops::Deref for Distances<T> {
    type Target = BTreeMap<(String, String), T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Distances<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: std::ops::Add<Output = T> + Clone + Copy + Ord> Distances<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert_ordered(&mut self, a: String, b: String, distance: T) {
        let key = if a < b {
            (a, b)
        } else {
            (b, a)
        };
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

    /// not sure if btree vs hashset is better here
    pub fn to_unique(&self) -> UniqueLocations {
        let mut set = BTreeSet::new();
        for (a, b) in self.keys() {
            set.insert(a.clone());
            set.insert(b.clone());
        }

        set
    }

    /// recursive
    pub fn find_shortest_path(&self, current: &str, remaining: &mut HashSet<&str>, total: T, shortest: &mut Option<T>) {
        // If no(thing)s remain, we've found a complete path
        if remaining.is_empty() {
            *shortest = match *shortest {
                None => Some(total),
                Some(s) => Some(s.min(total))
            };
            return;
        }

        // Try each remaining city as the next step
        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            // Get distance to this neighbor
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };
            let distance = self.get(&key).unwrap();
            
            // Visit this neighbor
            remaining.remove(next);
            self.find_shortest_path(
                next,
                remaining,
                total + *distance,
                shortest
            );
            remaining.insert(next);
        }
    }

    /// recursive
    pub fn find_longest_path(&self, current: &str, remaining: &mut HashSet<&str>, total: T, shortest: &mut Option<T>) {
        // If no(thing)s remain, we've found a complete path
        if remaining.is_empty() {
            *shortest = match *shortest {
                None => Some(total),
                Some(s) => Some(s.max(total))
            };
            return;
        }

        // Try each remaining city as the next step
        let neighbors: Vec<_> = remaining.iter().copied().collect();
        for next in neighbors {
            // Get distance to this neighbor
            let key = if current < next {
                (current.to_string(), next.to_string())
            } else {
                (next.to_string(), current.to_string())
            };
            let distance = self.get(&key).unwrap();
            
            // Visit this neighbor
            remaining.remove(next);
            self.find_longest_path(
                next,
                remaining,
                total + *distance,
                shortest
            );
            remaining.insert(next);
        }
    }

    pub fn get_unique_cities(&self) -> HashSet<&str> {
        let mut cities = HashSet::new();
        for ((city1, city2), _) in &self.0 {
            cities.insert(city1.as_str());
            cities.insert(city2.as_str());
        }
        cities
    }
}

pub fn distance() {}

pub type UniqueLocations = BTreeSet<String>;
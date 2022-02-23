use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Set<T> = BTreeSet<T>;
type Map<K, V> = BTreeMap<K, V>;

/// Represents set of available pairs.
#[derive(Debug)]
struct AvailablePairs<'a> {
    data: Map<&'a str, Set<&'a str>>,
}

impl<'a> AvailablePairs<'a> {
    /// Create set of all combinations of `names`.
    fn new(names: &[&'a str]) -> Self {
        let mut data = Map::new();
        let sorted_names = {
            let mut mutable_names = names.to_vec();
            mutable_names.sort_unstable();
            mutable_names
        };

        for &key_name in &sorted_names {
            let greater_names: Set<&str> = sorted_names
                .iter()
                .skip_while(|&&value_name| value_name <= key_name)
                .cloned()
                .collect();

            if !greater_names.is_empty() {
                data.insert(key_name, greater_names);
            }
        }

        Self { data }
    }

    /// Returns count of available pairs.
    fn count(&self) -> usize {
        self.data.iter().fold(0, |acc, (_, set)| acc + set.len())
    }

    /// Returns a pair made from `allowed_names`.
    fn get_pair(&self, allowed_names: &Set<&str>) -> Option<(&str, &str)> {
        self.data
            .iter()
            .find(|(&key, _)| allowed_names.contains(key))
            .and_then(|(&first_name, set)| {
                set.iter()
                    .find(|&name| allowed_names.contains(name))
                    .map(|&second_name| (first_name, second_name))
            })
    }

    /// Removes pair from set. Returns `true` if pair was found.
    ///
    /// `name_a` should be less than `name_b`.
    fn remove_pair(&mut self, name_a: &str, name_b: &str) -> bool {
        self.data
            .get_mut(name_a)
            .map(|set| set.remove(name_b))
            .unwrap_or(false)
    }

    /// Returns a pair made from `allowed_names` and removes from set.
    fn pop_pair(&mut self, allowed_names: &Set<&str>) -> Option<(&str, &str)> {
        let maybe_pair = self.get_pair(allowed_names);

        if let Some((a, b)) = maybe_pair {
            // self.remove_pair(a, b);
        }

        maybe_pair
    }
}

#[cfg(test)]
mod tests {
    mod available_pairs {
        use crate::*;

        #[test]
        fn new_dont_panic() {
            AvailablePairs::new(&["Alice", "Bob", "Zak"]);
        }

        #[test]
        fn count_for_3_names() {
            let pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(pt.count(), 3);
        }

        #[test]
        fn count_for_4_names() {
            let pt = AvailablePairs::new(&["Alice", "Bob", "Zak", "John"]);
            assert_eq!(pt.count(), 6);
        }

        #[test]
        fn count_for_10_names() {
            let pt = AvailablePairs::new(&["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"]);
            assert_eq!(pt.count(), 45);
        }

        #[test]
        fn get_pair_when_exists() {
            let pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(
                pt.get_pair(&Set::from(["Alice", "Bob"])),
                Some(("Alice", "Bob"))
            );
        }

        #[test]
        fn get_pair_when_impossible() {
            let pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(pt.get_pair(&Set::from(["Alice", "Jane"])), None);
        }

        #[test]
        fn remove_pair_when_exists() {
            let mut pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(pt.remove_pair("Alice", "Bob"), true);
        }

        #[test]
        fn remove_pair_when_impossible() {
            let mut pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(pt.remove_pair("Alice", "Bob"), true);
            assert_eq!(pt.remove_pair("Alice", "Bob"), false);
        }

        #[test]
        fn pop_pair_when_exists() {
            let mut pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(
                pt.pop_pair(&Set::from(["Alice", "Bob"])),
                Some(("Alice", "Bob"))
            );
            assert_eq!(pt.pop_pair(&Set::from(["Alice", "Bob"])), None);
        }

        #[test]
        fn pop_pair_when_impossible() {
            let mut pt = AvailablePairs::new(&["Alice", "Bob", "Zak"]);
            assert_eq!(pt.pop_pair(&Set::from(["Alice", "Jane"])), None);
        }
    }
}

fn main() {
    let ap = AvailablePairs::new(&["Alice", "Bob", "Jane", "Aaa"]);

    println!("ap: {:?}", ap);
}

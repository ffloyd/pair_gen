use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Set<T> = BTreeSet<T>;
type Map<K, V> = BTreeMap<K, V>;

/// Represents set of available pairs.
#[derive(Debug)]
struct AvailablePairs<T: Ord + Clone + Copy> {
    data: Map<T, Set<T>>,
}

impl<T: Ord + Clone + Copy> AvailablePairs<T> {
    /// Create set of all combinations of `names`.
    fn new(names: &[T]) -> Self {
        let mut data = Map::new();
        let sorted_names = {
            let mut mutable_names = names.to_vec();
            mutable_names.sort_unstable();
            mutable_names
        };

        for &key_name in &sorted_names {
            let greater_names: Set<T> = sorted_names
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
    fn get_pair(&self, allowed_names: &Set<T>) -> Option<(T, T)> {
        self.data
            .iter()
            .find(|(key, _)| allowed_names.contains(key))
            .and_then(|(&first_name, set)| {
                set.iter()
                    .find(|&name| allowed_names.contains(name))
                    .map(|&second_name| (first_name, second_name))
            })
    }

    /// Removes pair from set. Returns `true` if pair was found.
    ///
    /// `name_a` should be less than `name_b`.
    fn remove_pair(&mut self, name_a: T, name_b: T) -> bool {
        self.data
            .get_mut(&name_a)
            .map(|set| set.remove(&name_b))
            .unwrap_or(false)
    }

    /// Returns a pair made from `allowed_names` and removes from set.
    fn pop_pair(&mut self, allowed_names: &Set<T>) -> Option<(T, T)> {
        let maybe_pair = self.get_pair(allowed_names);

        if let Some((a, b)) = maybe_pair {
            self.remove_pair(a, b);
        }

        maybe_pair
    }

    /// Distributes `allowed_names` to available pairs.
    ///
    /// Excludes used names from `allowed_names`. Empty set will be returned if no single pair possible.
    fn pop_distribution(&mut self, allowed_names: &mut Set<T>) -> Set<(T, T)> {
        let mut result = Set::new();

        while let Some(pair) = self.pop_pair(allowed_names) {
            result.insert(pair);
            let (a, b) = pair;
            allowed_names.remove(&a);
            allowed_names.remove(&b);
        }

        result
    }
}

// 

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

        #[test]
        fn pop_distribution_when_possible() {
            let people = ["Alice", "Bob", "Jane", "John"];
            let mut pt = AvailablePairs::new(&people);
            assert_eq!(
                pt.pop_distribution(&mut Set::from(people)),
                Set::from([
                    ("Alice", "Bob"),
                    ("Jane", "John")
                ])
            );
        }

        #[test]
        fn pop_distribution_returns_unused() {
            let people = ["Alice", "Bob", "Jane"];
            let mut pt = AvailablePairs::new(&people);
            let mut ppl_set = Set::from(people);

            assert_eq!(
                pt.pop_distribution(&mut ppl_set),
                Set::from([
                    ("Alice", "Bob")
                ])
            );

            assert_eq!(ppl_set, Set::from(["Jane"]));
        }
    }
}

fn main() {
    let ap = AvailablePairs::new(&["Alice", "Bob", "Jane", "Aaa"]);

    println!("ap: {:?}", ap);
}

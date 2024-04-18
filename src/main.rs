use std::{
    collections::{BTreeSet as Set, VecDeque},
    fmt::Debug
};

// mod pair_rotation;

// possible dist (names, desires, history) -> Option<dist>
// best dist (names, desires, history)

/// Generates an ordered list of all possible pairs from `items`.
/// Elements in a pair are ordered.
fn all_pairs<T: Ord + Copy>(items: &Set<T>) -> Vec<(T, T)> {
    let mut result = Vec::new();
    let mut iter = items.iter();

    while let Some(&left_item) = iter.next() {
        iter.clone()
            .for_each(|&right_item| result.push((left_item, right_item)))
    }

    result
}

// Returns a pair distribution that uses only `allowed_pairs` and includes all `required_names`.
fn possible_distribution<T: Copy + Ord>(
    required_names: &Set<T>,
    allowed_pairs: &[(T, T)],
) -> Option<Vec<(T, T)>> {
    if required_names.is_empty() {
        return Some(Vec::new());
    }

    let mut iter = allowed_pairs.iter();

    while let Some(&pair) = iter.next() {
        let (left, right) = pair;

        if !(required_names.contains(&left) && required_names.contains(&right)) {
            continue;
        }

        let mut next_names = required_names.clone();
        next_names.remove(&left);
        next_names.remove(&right);

        if let Some(mut solution) = possible_distribution(&next_names, iter.as_slice()) {
            solution.push(pair);
            return Some(solution);
        }
    }

    None
}

#[derive(Clone, Debug)]
struct History<T> {
    data: VecDeque<Set<(T, T)>>,
}

impl<T: Ord + Copy> History<T> {
    fn new(history: &[&[(T, T)]]) -> Self {
        Self {
            data: history
                .iter()
                .map(|&dist| {
                    Set::from_iter(dist.iter().copied().map(|pair| {
                        if pair.0 < pair.1 {
                            pair
                        } else {
                            (pair.1, pair.0)
                        }
                    }))
                })
                .collect(),
        }
    }

    /// Returns all pairs in history.
    fn to_pairs(&self) -> Set<(T, T)> {
        let mut result = Set::new();
        self.data.iter().for_each(|set| result.extend(set.iter()));
        result
    }

    /// Forgets the earlier history entry.
    fn forget(&mut self) -> bool {
        self.data.pop_front().map(|_| true).unwrap_or(false)
    }

    /// Excludes pairs from history
    fn exclude(&mut self, pairs: &Set<(T, T)>) {
        self.data
            .iter_mut()
            .for_each(|dist| dist.retain(|x| pairs.contains(x)))
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

fn best_next_distribution<T: Ord + Copy + Debug>(
    required_names: &Set<T>,
    desires: &Set<(T, T)>,
    history: &History<T>,
) -> Vec<(T, T)> {
    let mut history = history.clone();
    // history.exclude(desires);

    let names_in_desires: Set<T> = desires.iter().flat_map(|&(a, b)| [a, b]).collect();
    let names_excluding_desires: Set<T> = required_names
        .iter()
        .filter(|&name| !names_in_desires.contains(name))
        .copied()
        .collect();

    loop {
        let disallowed_pairs = history.to_pairs();
        // println!("disallow pairs: {:?}", disallowed_pairs);
        let allowed_pairs: Vec<(T, T)> = all_pairs(&names_excluding_desires)
            .iter()
            .filter(|&pair| !disallowed_pairs.contains(pair))
            .copied()
            .collect();

        if let Some(mut solution) = possible_distribution(&names_excluding_desires, &allowed_pairs)
        {
            let mut result = Vec::from_iter(desires.iter().copied());

            result.append(&mut solution);

            return result;
        }

        if history.is_empty() {
            panic!("Impossible algorithm state");
        }
        println!("I have to forget 1 item in history...");
        history.forget();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all_pairs_for_3_items() {
        assert_eq!(
            all_pairs(&Set::from([1, 2, 3, 4])),
            vec![(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
        )
    }

    #[test]
    fn possible_distribution_for_4_items() {
        let names = Set::from([1, 2, 3, 4]);
        let pairs = all_pairs(&names);
        let result = possible_distribution(&names, &pairs);

        assert_eq!(result, Some(vec![(3, 4), (1, 2)]))
    }

    #[test]
    fn history_to_pairs() {
        let hist = History::new(&[&[(1, 2), (3, 4)], &[(1, 3), (2, 4)]]);

        let pairs = hist.to_pairs();

        assert_eq!(pairs, Set::from([(1, 2), (3, 4), (1, 3), (2, 4)]))
    }

    #[test]
    fn history_forget() {
        let mut hist = History::new(&[&[(1, 2), (3, 4)], &[(1, 3), (2, 4)]]);
        hist.forget();

        let pairs = hist.to_pairs();

        assert_eq!(pairs, Set::from([(1, 3), (2, 4)]))
    }

    #[test]
    fn best_next_distribution_simple_case() {
        let hist = History::new(&[&[(1, 2), (3, 4)], &[(1, 3), (2, 4)]]);
        let required_names = Set::from([1, 2, 3, 4]);
        let desires = Set::from([(1, 2)]);

        assert_eq!(
            best_next_distribution(&required_names, &desires, &hist),
            vec![(1, 2), (3, 4)]
        )
    }
}

pub fn main() {
    let history = History::new(&[
        &[("Jonas", "Iuri"), ("Rafael", "Roman"), ("Duarte", "Edin")],
        &[("Jonas", "Rafael"), ("Edin", "Roman"), ("Iuri", "NO ONE")],
        &[
            ("Jonas", "Roman"),
            ("Iuri", "Edin"),
            ("Rafael", "Duarte"),
            ("Dejan", "Max"),
        ],
        &[
            ("Reza", "Roman"),
            ("Max", "Rafael"),
            ("Edin", "Jonas"),
            ("Dejan", "Erik"),
            ("Iuri", "NO ONE"),
        ],
        &[
            ("Dejan", "Reza"),
            ("Edin", "Rafael"),
            ("Erik", "Jonas"),
            ("Max", "Roman"),
            ("Iuri", "NO ONE"),
        ],
        &[
            ("Dejan", "Jonas"),
            ("Edin", "Reza"),
            ("Erik", "Iuri"),
            ("Moritz", "Roman"),
            ("Tuomo", "NO ONE"),
        ],
        &[
            ("Dejan", "Roman"),
            ("Edin", "Tuomo"),
            ("Erik", "Rafael"),
            ("Jonas", "Moritz"),
            ("Max", "Reza"),
        ],
        &[
            ("Dejan", "Edin"),
            ("Erik", "Roman"),
            ("Iuri", "Reza"),
            ("Jonas", "Max"),
            ("Moritz", "Tuomo"),
            ("Rafael", "NO ONE"),
        ],
        &[
            ("Dejan", "Rafael"),
            ("Edin", "Moritz"),
            ("Erik", "Max"),
            ("Reza", "Roman"),
            ("Roman", "Tuomo"),
            ("Reza", "Tuomo"),
        ],
        &[
            ("Dejan", "Edin"),
            ("Erik", "Moritz"),
            ("Iuri", "Roman"),
            ("Jonas", "Reza"),
            ("Max", "Tuomo"),
        ],
        &[
            ("Dejan", "Tuomo"),
            ("Edin", "Max"),
            ("Iuri", "Roman"),
            ("Moritz", "Reza"),
            ("Ema", "Moritz"),
            ("Ema", "Reza"),
        ],
        &[
            ("Dejan", "Reza"),
            ("Edin", "Roman"),
            ("Ema", "Erik"),
            ("Iuri", "Tuomo"),
            ("Max", "Moritz"),
        ],
        &[
            ("Dejan", "Moritz"),
            ("Edin", "Ema"),
            ("Erik", "Reza"),
            ("Jonas", "Tuomo"),
            ("Max", "Roman"),
        ],
        &[
            ("Dejan", "Max"),
            ("Edin", "Reza"),
            ("Ema", "Tuomo"),
            ("Jonas", "Roman"),
        ],
        &[("Dejan", "Roman"), ("Edin", "Jonas"), ("Rafael", "Reza")],
        &[("Dejan", "Edin"), ("NO ONE", "Rafael"), ("Reza", "Roman")],
        &[("Dejan", "Rafael"), ("Edin", "Roman"), ("Jonas", "Reza")],
        &[("Dejan", "Jonas"), ("Edin", "Reza"), ("Rafael", "Roman")],
        &[("Dejan", "Reza"), ("Edin", "Rafael"), ("Jonas", "Roman")],
        &[("Dejan", "Roman"), ("Edin", "Jonas"), ("Rafael", "Reza")],
        &[("Dejan", "Edin"), ("Jonas", "Rafael"), ("Reza", "Roman")],
        // Add pairs here to disallow
        &[("Ema", "Moritz")],
    ]);

    let names = Set::from([
        "Dejan", "Edin", "Roman", "Reza", "Jonas", "Max", "Tuomo",
        "Ema",
        // "Iuri",  "Rafael", "Moritz", "Erik",
    ]);

    // let platf = Set::from(["Jonas", "Dejan", "Edin", "Roman", "Reza", "Rafael"]);
    let platf = Set::from(["Jonas", "Dejan", "Edin", "Roman", "Reza", "Rafael"]);

    let exp = Set::from(["Erik", "Iuri", "Max", "Moritz", "Tuomo", "Ema"]);

    let desires = Set::from([]);

    let mut result = best_next_distribution(&platf, &desires, &history);
    result.sort_unstable();

    println!("result: {:?}", result);
}

use std::collections::BTreeSet as Set;

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
}

pub fn main() {}

use std::collections::BTreeSet as Set;

// ideas:
//
// simple recursive distribution generator is done
// but we can do a more challenging task
// to generate all possible distributions
//
// API can be the following:
// let ctx = Context::new(names: &[T], desires: &[(T, T)], history: &[&[(T, T)]])
// Context - contains pair distribution context.
//
// ctx.next_dist() -> Vec<(T, T)> - Returns next distribution.
// ctx.all_next_dist() -> DistTree - Returns all possible distributions as a tree-like structure.
// count_dists(DistTree)

fn all_pairs<T: Clone + Ord + Copy>(names: Set<T>) -> Vec<(T, T)> {
    let mut result = Vec::<(T, T)>::new();

    let mut iter = names.iter();
    while let Some(&left_name) = iter.next() {
        iter.clone().for_each(|&right_name| {
            result.push((left_name, right_name));
        })
    }

    result
}

fn dist<T: Ord + Clone + Copy>(pairs: &[(T, T)], names: Set<T>) -> Option<Vec<(T, T)>> {
    if names.is_empty() {
        return Some(Vec::new());
    }
    
    let mut iter = pairs.iter();
    while let Some(pair) = iter.next() {
        let (a, b) = pair;
        if names.contains(a) && names.contains(b) {
            let pairs_left = iter.clone().as_slice();

            let mut next_names = names.clone();
            next_names.remove(a);
            next_names.remove(b);
            
            if let Some(mut tail) = dist(pairs_left, next_names) {
                tail.push(*pair);
                return Some(tail);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn all_pairs_for_3_names() {
        assert_eq!(
            all_pairs(Set::from([1, 2, 3])),
            vec![
                (1, 2),
                (1, 3),
                (2, 3)
            ]
        )
    }

    #[test]
    fn dist_for_4_names() {
        let names = Set::from([1, 2, 3, 4]);
        let pairs = all_pairs(names.clone());
        let pairs_slice = pairs.iter().as_slice();
        assert_eq!(
            dist(pairs_slice, names),
            Some(vec![(3, 4), (1, 2)])
        )
    }
}

fn dist2<T: Clone + Ord + Copy>(pairs: Set<(T, T)>, names: Set<T>) -> Option<Vec<(T, T)>> {
    let pairs_vec = Vec::from_iter(pairs.iter().cloned());
    let pairs_slice = pairs_vec.iter().as_slice();

    dist(pairs_slice, names)
}

fn main() {
    let names = Set::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    let pairs = all_pairs(names.clone());
    let mut pairs_set = Set::from_iter(pairs.iter().cloned());

    while let Some(new_dist) = dist2(pairs_set.clone(), names.clone()) {
        println!("{:?}", new_dist);

        for pair in new_dist {
            pairs_set.remove(&pair);
        }
    }
}

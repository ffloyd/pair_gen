use std::collections::BTreeMap;
use std::collections::BTreeSet;

struct Context {
    people_map: BTreeMap<String, BTreeSet<String>>,
    next_distribution: BTreeSet<BTreeSet<String>>,
}

impl Context {
    fn from_vec(people: Vec<String>) -> Self {
        let mut people_map = BTreeMap::new();

        for human in people.clone() {
            people_map.insert(
                human.clone(),
                BTreeSet::from_iter(people.iter().filter(|&x| *x != human).cloned()),
            );
        }

        let next_distribution = BTreeSet::new();

        return Self {
            people_map,
            next_distribution,
        };
    }

    fn new(people: &[&str]) -> Self {
        let people_as_strings: Vec<String> = people.iter().map(|x| x.to_string()).collect();

        return Self::from_vec(people_as_strings);
    }

    fn register_group(&mut self, group: &[&str]) {
        let group_as_strings: Vec<String> = group.iter().map(|x| x.to_string()).collect();

        self.register_group_vec(group_as_strings);
    }

    fn register_group_vec(&mut self, group: Vec<String>) {
        for human_a in group.clone() {
            for human_b in group.clone() {
                self.people_map
                    .get_mut(&human_a)
                    .expect("AAA!!!")
                    .remove(&human_b);
            }
        }

        self.next_distribution.insert(BTreeSet::from_iter(group.clone().iter().cloned()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_context() {
        let result = Context::new(&["Alice", "Alex", "Bob"]);

        assert_eq!(
            result.people_map,
            BTreeMap::from([
                (
                    "Alice".to_string(),
                    BTreeSet::from(["Alex".to_string(), "Bob".to_string()])
                ),
                (
                    "Alex".to_string(),
                    BTreeSet::from(["Alice".to_string(), "Bob".to_string()])
                ),
                (
                    "Bob".to_string(),
                    BTreeSet::from(["Alice".to_string(), "Alex".to_string()])
                ),
            ])
        )
    }

    #[test]
    fn register_group() {
        let mut context = Context::new(&["Alice", "Alex", "Bob"]);

        context.register_group(&["Alice", "Bob"]);

        assert_eq!(
            context.people_map,
            BTreeMap::from([
                ("Alice".to_string(), BTreeSet::from(["Alex".to_string()])),
                (
                    "Alex".to_string(),
                    BTreeSet::from(["Alice".to_string(), "Bob".to_string()])
                ),
                ("Bob".to_string(), BTreeSet::from(["Alex".to_string()]))
            ])
        );

        assert!(context
            .next_distribution
            .contains(&BTreeSet::from(["Alice".to_string(), "Bob".to_string()])))
    }
}

fn main() {
    println!("Hello world!");
}

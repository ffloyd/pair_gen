use std::collections::BTreeMap;
use std::collections::BTreeSet;

type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;

#[derive(Debug)]
struct Context {
    people: Vec<String>,
    people_map: Map<String, Set<String>>,
    next_distribution: Set<Set<String>>,
}

impl Context {
    fn from_vec(people: Vec<String>) -> Self {
        let mut people_map = Map::new();

        for human in people.clone() {
            people_map.insert(
                human.clone(),
                Set::from_iter(people.iter().filter(|&x| *x > human).cloned()),
            );
        }

        let next_distribution = Set::new();

        Self {
            people,
            people_map,
            next_distribution,
        }
    }

    fn new(people: &[&str]) -> Self {
        let people_as_strings: Vec<String> = people.iter().map(|x| x.to_string()).collect();

        Self::from_vec(people_as_strings)
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

        self.next_distribution
            .insert(Set::from_iter(group.iter().cloned()));
    }

    fn build_next_distribution(&mut self) {
        let mut people_left = Set::from_iter(self.people.iter().cloned());
        while !people_left.is_empty() {
            let human_a = people_left.iter().cloned().next().expect("AAA");
            people_left.remove(&human_a);

            match people_left.iter().cloned().next() {
                None => self.register_group(&[&human_a]),
                Some(human_b) => {
                    self.register_group(&[&human_a, &human_b]);
                    people_left.remove(&human_b);
                }
            }
        }
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
            Map::from([
                (
                    "Alex".to_string(),
                    Set::from(["Alice".to_string(), "Bob".to_string()])
                ),
                ("Alice".to_string(), Set::from(["Bob".to_string()])),
                ("Bob".to_string(), Set::from([])),
            ])
        )
    }

    #[test]
    fn register_group() {
        let mut context = Context::new(&["Alice", "Alex", "Bob"]);

        context.register_group(&["Alice", "Bob"]);

        assert_eq!(
            context.people_map,
            Map::from([
                ("Alice".to_string(), Set::from([])),
                (
                    "Alex".to_string(),
                    Set::from(["Alice".to_string(), "Bob".to_string()])
                ),
                ("Bob".to_string(), Set::from([]))
            ])
        );

        assert!(context
            .next_distribution
            .contains(&Set::from(["Alice".to_string(), "Bob".to_string()])))
    }

    #[test]
    fn builds_next_distribution() {
        let mut context = Context::new(&["Alice", "Alex", "Bob"]);

        context.build_next_distribution();

        assert_eq!(
            context.next_distribution,
            Set::from([
                Set::from(["Alex".to_string(), "Alice".to_string()]),
                Set::from(["Bob".to_string()])
            ])
        );
    }
}

fn main() {
    let mut ctx = Context::new(&["Alex", "Alice", "Bob"]);
    ctx.build_next_distribution();

    println!("{:?}", ctx);
}

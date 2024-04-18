use std::collections::BTreeSet as Set;

pub struct PairRotation<T> {
    history: Vec<Set<(T, T)>>,
}

impl<T> PairRotation<T> {
    pub fn new(history: &[&[(T, T)]]) -> Self {
        todo!()
    }
}

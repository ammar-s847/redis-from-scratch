

pub mod sorted_set;

#[derive(Debug)]
pub struct SortedSet<T> {
    elements: Vec<T>,
}

impl SortedSet {
    fn new() -> SortedSet {
        SortedSet {
            elements: Vec::new(),
        }
    }

    fn insert(&mut self, element: T) {
        self.elements.push(element);
        self.elements.sort();
    }

    fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }
}

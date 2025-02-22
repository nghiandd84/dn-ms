use std::collections::HashMap;

#[derive(Clone)]
pub struct Registry<I> {
    items: HashMap<String, I>,
}

impl<I> Registry<I> {
    pub fn build() -> Self {
        Registry {
            items: HashMap::new(),
        }
    }
}

impl<I> Registry<I> {
    pub fn get(&self, key: &str) -> Option<&I> {
        self.items.get(key)
    }

    pub fn add(&mut self, key: String, item: I) {
        self.items.insert(key, item);
    }
}

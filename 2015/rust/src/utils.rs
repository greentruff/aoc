use std::collections::HashMap;

#[derive(Debug)]
pub struct StringToId {
    known: HashMap<String, usize>,
}
impl StringToId {
    pub fn new() -> Self {
        StringToId {
            known: HashMap::new(),
        }
    }
    pub fn get(&mut self, str: &str) -> usize {
        let next_id = self.known.len();
        *self.known.entry(str.to_string()).or_insert(next_id)
    }

    pub fn len(&self) -> usize {
        self.known.len()
    }
}

use std::collections::HashMap;
use std::hash::Hash;
use std::num::TryFromIntError;

#[derive(Debug)]
pub struct StringToId<T> {
    known: HashMap<String, T>,
}
impl<T: Eq + Hash + TryFrom<usize> + Into<usize> + Copy> StringToId<T>
where
    TryFromIntError: From<<T as TryFrom<usize>>::Error>,
{
    pub fn new() -> Self {
        StringToId { known: HashMap::new() }
    }
    pub fn get(&mut self, str: &str) -> Result<T, TryFromIntError> {
        let next_id: T = self.known.len().try_into()?;
        Ok(*self.known.entry(str.to_string()).or_insert(next_id))
    }

    pub fn len(&self) -> usize {
        self.known.len()
    }
}

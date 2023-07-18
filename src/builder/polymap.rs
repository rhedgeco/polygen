use fxhash::FxHashSet;

pub struct PolyMap<T> {
    keys: FxHashSet<String>,
    values: Vec<T>,
}

impl<T> Default for PolyMap<T> {
    fn default() -> Self {
        Self {
            keys: Default::default(),
            values: Default::default(),
        }
    }
}

impl<T> PolyMap<T> {
    pub fn insert(&mut self, key: String, value: T) -> bool {
        if self.keys.insert(key) {
            self.values.push(value);
            return true;
        }

        false
    }

    pub fn as_slice(&self) -> &[T] {
        &self.values
    }
}

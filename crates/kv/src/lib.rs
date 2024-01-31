use std::collections::HashMap;

pub struct KvStores<V> {
    stores: HashMap<String, KvStore<V>>,
}

pub struct KvStore<V> {
    namespace: String,
    values: HashMap<String, V>,
}

impl<V> KvStores<V> {
    pub fn new() -> Self {
        Self {
            stores: HashMap::new(),
        }
    }

    pub fn new_store(&mut self, namespace: &str) {
        if self.stores.contains_key(namespace) {
            return;
        }

        self.stores.insert(
            namespace.into(),
            KvStore {
                namespace: namespace.into(),
                values: HashMap::new(),
            },
        );
    }

    pub fn get_store(&self, namespace: &str) -> Option<&KvStore<V>> {
        self.stores.get(namespace)
    }

    pub fn store(&mut self, namespace: &str, key: &str, value: V) {
        if !self.stores.contains_key(namespace) {
            self.new_store(namespace);
        }

        let store = self.stores.get_mut(namespace).unwrap();
        store.values.insert(key.into(), value);
    }

    pub fn get(&self, namespace: &str, key: &str) -> Option<&V> {
        let store = &self.get_store(namespace)?;
        store.values.get(key)
    }
}

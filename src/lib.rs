use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use std::collections::HashMap;

pub struct MutexesMap<K, V, S = std::collections::hash_map::RandomState> {
    base: Arc<Mutex<HashMap<K, V, S>>>,
}

pub struct MutexesMapGuard<'a, K, V, S = std::collections::hash_map::RandomState> {
    base: MutexGuard<'a, V>,
}

impl<K, V, S> MutexesMap<K, V, S> {
    pub fn new() {
        Self {
            base: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn lock(&self, key: K) -> LockResult<MutexGuard<'_, V>> {
        let this = self.lock().unwrap();

        if let Some(m) = this.base.get(key) {
            m.lock().unwrap();
            m.clone()
        } else {
            let m = Arc::new(Mutex::new(()));
            this.base.insert(key, m.clone());
            m.clone()
        }
    }
    // TODO: try_lock()
}

impl<'a, K, V, S> MutexesMapGuard<'a, K, V, S> {
    // type Target = V; // TODO
}

impl<'a, K, V, S> From<MutexesMapGuard<'a, K, V, S>> for MutexGuard<'a, V> {

}
use std::collections::btree_map::OccupiedEntry;
use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use std::collections::HashMap;

// TODO: (Low priority) Support other random states S.
pub struct MutexesMap<K>
where
    K: Hash + Eq, 
{
    pub(crate) base: Arc<Mutex<HashMap<K, Arc<Mutex<()>>>>>,
}

pub struct MutexesMapGuard<'a, K>
where
    K: Hash + Eq, 
{
    map: &'a MutexesMap<K>,
    guard: &'a MutexGuard<'a, ()>,
    key: K
}

impl<K> MutexesMap<K>
where
    K: Hash + Eq + Copy, 
{
    pub fn new() -> Self {
        Self {
            base: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn lock<'a>(&'a self, key: K) -> LockResult<&'a MutexesMapGuard<'a, K>> {
        let this: &'a _ = &self.base;//.clone().lock().unwrap();
        let this: &'a _ = &this.lock().unwrap();

        // let inner_guard = &this.lock().unwrap().entry(key)/*.or_insert(Arc::new(Mutex::new(())))*/.clone().lock().unwrap();
        let inner_guard = &this.get(&key).unwrap().clone().lock().unwrap();

        Ok(&MutexesMapGuard {
            map: &self,
            guard: inner_guard,
            key,
        })
        // let guard = inner_mutex.lock();
        // match *inner_guard {
        //     Ok(guard) => Ok(MutexesMapGuard {
        //         map: &self,
        //         guard: guard,
        //         key,
        //     }),
        //     Err(e) => Err(PoisonError::new(MutexesMapGuard {
        //         map: &self,
        //         guard: e.into_inner(),
        //         key,
        //     })),
        // }
        
    }
    // TODO: try_lock()
}

impl<'a, K> MutexesMapGuard<'a, K>
where
    K: Hash + Eq, 
{
    // type Target = (); // TODO
}

impl<'a, K> From<MutexesMapGuard<'a, K>> for &'a MutexGuard<'a, ()>
where
    K: Hash + Eq, 
{
    fn from(guard: MutexesMapGuard<'a, K>) -> Self {
        guard.guard // FIXME: unwrap()
    }
}

impl<'a, K> Drop for MutexesMapGuard<'a, K>
where
    K: Hash + Eq, 
{
    fn drop(&mut self) {
        // let mut this = self.lock().unwrap();
        self.map.base.lock().unwrap().remove(&self.key);
        // Here inner guard drops.
    }
}

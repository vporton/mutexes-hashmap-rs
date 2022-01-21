use std::collections::hash_map::Entry;
use std::hash::Hash;
use std::sync::{Arc, LockResult, Mutex, MutexGuard, PoisonError};
use std::collections::HashMap;

// TODO: (Low priority) Support other random states S.
pub struct MutexesMap<K>
where
    K: Hash + Eq, 
{
    pub(crate) base: Arc<Mutex<HashMap<K, Mutex<()>>>>,
}

pub struct MutexesMapGuard<'a, K>
where
    K: Hash + Eq, 
{
    map: &'a MutexesMap<K>,
    guard: MutexGuard<'a, ()>,
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
    pub fn lock(&self, key: K) -> LockResult<MutexesMapGuard<'_, K>> {
        let mut this = self.base.lock().unwrap();

        let inner_guard = match this.entry(key) {
            Entry::Occupied(m) => m.into_mut().lock().unwrap(),
            Entry::Vacant(v) => {
                let m = Mutex::new(());
                v.insert(m).lock().unwrap()    
            }
        };
    
        Ok(MutexesMapGuard {
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

impl<'a, K> From<MutexesMapGuard<'a, K>> for MutexGuard<'a, ()>
where
    K: Hash + Eq, 
{
    fn from(guard: MutexesMapGuard<'a, K>) -> Self {
        guard.guard
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

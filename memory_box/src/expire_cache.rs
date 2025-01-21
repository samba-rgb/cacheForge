// use std::collections::HashMap;
// use std::time::{Duration, Instant};
// use tokio::sync::Mutex;
// use tokio::time::sleep;
// use std::sync::Arc;

// #[derive(Clone)]
// struct ExpiringValue<V> {
//     value: V,
//     expiry: Instant,
// }

// pub struct ExpireCache<K, V> {
//     map: Arc<Mutex<HashMap<K, ExpiringValue<V>>>>,
// }

// impl<K: std::hash::Hash + Eq + Clone + Send + 'static, V: Clone + Send + 'static> ExpireCache<K, V> {
//     pub fn new() -> Self {
//         let cache = Self {
//             map: Arc::new(Mutex::new(HashMap::new())),
//         };

//         let map_clone = cache.map.clone();
//         tokio::spawn(async move {
//             loop {
//                 sleep(Duration::new(10, 0)).await;
//                 let mut map = map_clone.lock().await;
//                 let now = Instant::now();
//                 map.retain(|_, v| v.expiry > now);
//             }
//         });

//         cache
//     }

//     pub async fn insert(&self, key: K, value: V, ttl: usize) {
//         let expiry = Instant::now() + Duration::from_secs(ttl as u64);
//         let expiring_value = ExpiringValue { value, expiry };

//         let mut map = self.map.lock().await;
//         map.insert(key, expiring_value);
//     }

//     pub async fn get(&self, key: &K) -> Option<V> {
//         let mut map = self.map.lock().await;
//         if let Some(expiring_value) = map.get(key) {
//             if expiring_value.expiry > Instant::now() {
//                 return Some(expiring_value.value.clone());
//             } else {
//                 map.remove(key);
//             }
//         }
//         None
//     }

//     pub async fn remove(&self, key: &K) -> Option<V> {
//         let mut map = self.map.lock().await;
//         map.remove(key).map(|expiring_value| expiring_value.value)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::time::Duration;

//     #[tokio::test]
//     async fn test_insert_and_get() {
//         let cache = ExpireCache::new();
//         cache.insert("key1", "value1", Duration::new(5, 0)).await;

//         sleep(Duration::new(3, 0)).await;
//         assert_eq!(cache.get(&"key1").await, Some("value1"));

//         sleep(Duration::new(3, 0)).await;
//         assert_eq!(cache.get(&"key1").await, None);
//     }

//     #[tokio::test]
//     async fn test_remove() {
//         let cache = ExpireCache::new();
//         cache.insert("key1", "value1", Duration::new(5, 0)).await;

//         sleep(Duration::new(1, 0)).await;
//         assert_eq!(cache.remove(&"key1").await, Some("value1"));
//         assert_eq!(cache.get(&"key1").await, None);
//     }

//     #[tokio::test]
//     async fn test_expiry() {
//         let cache = ExpireCache::new();
//         cache.insert("key1", "value1", Duration::new(2, 0)).await;

//         sleep(Duration::new(3, 0)).await;
//         assert_eq!(cache.get(&"key1").await, None);
//     }

//     #[tokio::test]
//     async fn test_multiple_keys() {
//         let cache = ExpireCache::new();
//         cache.insert("key1", "value1", Duration::new(5, 0)).await;
//         cache.insert("key2", "value2", Duration::new(3, 0)).await;

//         sleep(Duration::new(2, 0)).await;
//         assert_eq!(cache.get(&"key1").await, Some("value1"));
//         assert_eq!(cache.get(&"key2").await, Some("value2"));

//         sleep(Duration::new(2, 0)).await;
//         assert_eq!(cache.get(&"key1").await, Some("value1"));
//         assert_eq!(cache.get(&"key2").await, None);

//         sleep(Duration::new(2, 0)).await;
//         assert_eq!(cache.get(&"key1").await, None);
//     }
// }
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration as StdDuration, Instant};

#[derive(Clone)]
struct ExpiringValue<V> {
    value: V,
    expiry: Instant,
}

pub struct ExpireCache<K, V> {
    map: RwLock<HashMap<K, ExpiringValue<V>>>,
}

impl<K, V> ExpireCache<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    /// Creates a new `ExpireCache` instance.
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    /// Inserts a key-value pair with a time-to-live (TTL) in seconds.
    ///
    /// # Panics
    /// Panics if the TTL exceeds 60 seconds.
    pub fn insert(&self, key: K, value: V, ttl: usize) {
        if ttl > 60 {
            panic!("TTL cannot exceed 60 seconds");
        }

        let expiry = Instant::now() + StdDuration::from_secs(ttl as u64);
        let expiring_value = ExpiringValue { value, expiry };

        // Insert the value into the cache.
        {
            let mut map = self.map.write().unwrap();
            map.insert(key, expiring_value);
        }

        // Clean up expired entries after the insertion.
        self.clean_expired();
    }

    /// Retrieves the value associated with a key, if it has not expired.
    pub fn get(&self, key: &K) -> Option<V> {
        // Clean up expired entries before attempting retrieval.
        self.clean_expired();

        // Retrieve the value from the cache.
        let map = self.map.read().unwrap();
        map.get(key).and_then(|v| {
            if v.expiry > Instant::now() {
                Some(v.value.clone())
            } else {
                None
            }
        })
    }

    fn clean_expired(&self) {
        let mut map = self.map.write().unwrap();
        let now = Instant::now();
        map.retain(|_, v| v.expiry > now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_expire_cache() {
        let cache = ExpireCache::new();

        cache.insert("key1", "value1", 2); 
        cache.insert("key2", "value2", 1);

        assert_eq!(cache.get(&"key1"), Some("value1"));
        assert_eq!(cache.get(&"key2"), Some("value2"));

        sleep(StdDuration::from_secs(1));
        assert_eq!(cache.get(&"key1"), Some("value1")); 
        assert_eq!(cache.get(&"key2"), None);         

        sleep(StdDuration::from_secs(1));
        assert_eq!(cache.get(&"key1"), None);          
    }
}

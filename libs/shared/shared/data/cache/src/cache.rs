use redis::{Client, Commands, RedisResult};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Display;
use std::marker::PhantomData;
use std::time::Duration;
// Re-import the Cache struct from the previous immersive
// In a real project, this would be in your `lib.rs` or a separate module.
// For this example, we'll include the necessary parts again for self-containment.

/// A generic, thread-safe cache that uses Redis as its backend.
///
/// This cache leverages Redis for storage, expiration (TTL), and concurrent access.
/// Keys and values are serialized to JSON before being stored in Redis.

#[derive(Clone)] 
pub struct Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Serialize + DeserializeOwned + Display,
    V: Clone + Serialize + DeserializeOwned,
{
    /// The Redis client used to connect to the Redis server.
    client: Client,
    /// An optional default time-to-live for cache entries.
    default_ttl: Option<Duration>,
    /// A prefix for all keys stored in Redis to avoid conflicts.
    key_prefix: String,

    /// PhantomData to mark that V is used, even if not directly stored in a field.
    /// This is necessary because the actual values (V) are stored in Redis,
    /// not directly within this struct, but the type parameter V is still
    /// essential for the `get` and `insert` methods' type signatures.
    _phantom_k: PhantomData<K>,
    _phantom_v: PhantomData<V>,
}

impl<K, V> Cache<K, V>
where
    K: Eq + std::hash::Hash + Clone + Serialize + DeserializeOwned + Display,
    V: Clone + Serialize + DeserializeOwned,
{
    /// Creates a new `Cache` instance connected to a Redis server.
    ///
    /// # Arguments
    /// * `redis_url`: The URL of the Redis server (e.g., "redis://127.0.0.1/").
    /// * `key_prefix`: A string prefix to prepend to all keys to avoid conflicts
    ///   with other data in Redis.
    ///
    /// # Returns
    /// A `RedisResult<Self>` which is `Ok(Cache)` on success or `Err(redis::Error)` on failure.
    pub fn new(redis_url: &str, key_prefix: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Cache {
            client,
            default_ttl: None,
            key_prefix: key_prefix.to_string(),
            _phantom_k: PhantomData, // Initialize PhantomData
            _phantom_v: PhantomData
        })
    }

    /// Creates a new `Cache` instance with a specified default time-to-live (TTL).
    ///
    /// # Arguments
    /// * `redis_url`: The URL of the Redis server.
    /// * `key_prefix`: A string prefix for all keys.
    /// * `default_ttl`: The duration after which a cache entry will expire if no
    ///   specific TTL is provided during insertion.
    ///
    /// # Returns
    /// A `RedisResult<Self>` which is `Ok(Cache)` on success or `Err(redis::Error)` on failure.
    pub fn with_default_ttl(
        redis_url: &str,
        key_prefix: &str,
        default_ttl: Duration,
    ) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Cache {
            client,
            default_ttl: Some(default_ttl),
            key_prefix: key_prefix.to_string(),
            _phantom_k: PhantomData, // Initialize PhantomData
            _phantom_v: PhantomData
        })
    }

    /// Helper to get a connection from the client.
    fn get_connection(&self) -> RedisResult<redis::Connection> {
        self.client.get_connection()
    }

    /// Constructs the full Redis key with the configured prefix.
    fn get_full_key(&self, key: &K) -> String {
        format!("{}:{}", self.key_prefix, key)
    }

    /// Inserts a key-value pair into the cache with an optional specific TTL.
    ///
    /// If `ttl` is `None`, the `default_ttl` of the cache will be used.
    /// If both are `None`, the entry will never expire.
    ///
    /// # Arguments
    /// * `key`: The key to insert.
    /// * `value`: The value associated with the key.
    /// * `ttl`: An optional `Duration` for this specific entry's time-to-live.
    ///
    /// # Returns
    /// A `RedisResult<()>` indicating success or failure.
    pub fn insert(&self, key: K, value: V, ttl: Option<Duration>) -> RedisResult<()> {
        let mut conn = self.get_connection()?;
        let full_key = self.get_full_key(&key);
        let serialized_value = serde_json::to_string(&value)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Serialization failed", e.to_string())))?;

        let actual_ttl = ttl.or(self.default_ttl);

        match actual_ttl {
            Some(duration) => {
                let _: () = conn.set_ex(&full_key, serialized_value, duration.as_secs() as usize)?;
            }
            None => {
                let _: () = conn.set(&full_key, serialized_value)?;
            }
        }
        Ok(())
    }

    /// Retrieves the value associated with the given key.
    ///
    /// Returns `Some(V)` if the key exists and the entry has not expired,
    /// otherwise returns `None`. Redis automatically handles expiration.
    ///
    /// # Arguments
    /// * `key`: The key to retrieve.
    ///
    /// # Returns
    /// A `RedisResult<Option<V>>` which is `Ok(Some(V))` if found, `Ok(None)` if not found,
    /// or `Err(redis::Error)` on Redis communication or deserialization failure.
    pub fn get(&self, key: &K) -> RedisResult<Option<V>> {
        let mut conn = self.get_connection()?;
        let full_key = self.get_full_key(key);
        let result: Option<String> = conn.get(&full_key)?;

        match result {
            Some(serialized_value) => {
                let value: V = serde_json::from_str(&serialized_value)
                    .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Deserialization failed", e.to_string())))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Removes a key-value pair from the cache.
    ///
    /// # Arguments
    /// * `key`: The key to remove.
    ///
    /// # Returns
    /// A `RedisResult<bool>` which is `Ok(true)` if the key was removed, `Ok(false)` if not found,
    /// or `Err(redis::Error)` on Redis communication failure.
    pub fn remove(&self, key: &K) -> RedisResult<bool> {
        let mut conn = self.get_connection()?;
        let full_key = self.get_full_key(key);
        let count: i64 = conn.del(&full_key)?;
        Ok(count > 0)
    }

    /// Clears all entries from the cache that match the configured key prefix.
    ///
    /// **Warning:** This operation can be slow on large databases as it involves
    /// iterating over keys. For a very large number of keys, consider using
    /// `SCAN` in batches.
    ///
    /// # Returns
    /// A `RedisResult<()>` indicating success or failure.
    pub fn clear(&self) -> RedisResult<()> {
        let mut conn = self.get_connection()?;
        let pattern = format!("{}*", self.key_prefix);
        let keys: Vec<String> = conn.keys(&pattern)?;
        if !keys.is_empty() {
            let _: () = conn.del(keys)?;
        }
        Ok(())
    }

    /// Returns the number of active entries in the cache that match the prefix.
    ///
    /// **Warning:** This operation can be slow on large databases as it involves
    /// iterating over keys.
    ///
    /// # Returns
    /// A `RedisResult<usize>` which is `Ok(count)` on success or `Err(redis::Error)` on failure.
    pub fn len(&self) -> RedisResult<usize> {
        let mut conn = self.get_connection()?;
        let pattern = format!("{}*", self.key_prefix);
        let keys: Vec<String> = conn.keys(&pattern)?;
        Ok(keys.len())
    }

    /// Returns `true` if the cache contains no entries matching the prefix.
    ///
    /// # Returns
    /// A `RedisResult<bool>` which is `Ok(true)` if empty, `Ok(false)` otherwise,
    /// or `Err(redis::Error)` on Redis communication failure.
    pub fn is_empty(&self) -> RedisResult<bool> {
        Ok(self.len()? == 0)
    }
}

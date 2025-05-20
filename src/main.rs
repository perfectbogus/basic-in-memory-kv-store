use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum DbError {
    KeyNotFound,
    SerializationError(String),
    StorageError(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::KeyNotFound => write!(f, "Key not found"),
            DbError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            DbError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl Error for DbError {}

type DbResult<T> = Result<T, DbError>;

struct MemoryDb<K, V> {
    data: HashMap<K, V>,
}
impl<K, V> MemoryDb<K, V>
where 
    K: std::cmp::Eq + std::hash::Hash + Clone,
    V: Clone,
{
    fn new() -> Self {
        MemoryDb {
            data: HashMap::new()
        }
    }
    
    fn insert(&mut self, key: K, value: V) -> DbResult<()> {
        self.data.insert(key, value);
        Ok(())
    }
    
    fn get(&self, key: &K) -> DbResult<V> {
        self.data.get(key).cloned().ok_or(DbError::KeyNotFound)
    }
    
    fn delete(&mut self, key: &K) -> DbResult<()> {
        if self.data.remove(key).is_none() {
            Ok(())
        } else {
            Err(DbError::KeyNotFound)
        }
    }
    
}












fn main() {
    println!("Hello, world!");
}

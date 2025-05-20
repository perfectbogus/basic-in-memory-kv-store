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
    
    fn set(&mut self, key: K, value: V) -> DbResult<()> {
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
    
    fn contains(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    
    fn keys(&self) -> Vec<K> {
        self.data.keys().cloned().collect()
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
}

fn main() {
    let mut db = MemoryDb::<String, String>::new();
    
    db.set(String::from("name"), String::from("Rust Database")).unwrap();
    db.set(String::from("version"), String::from("0.1.0")).unwrap();
    
    match db.get(&String::from("name")) {
        Ok(value) => println!("Name: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    let key = String::from("version");
    if db.contains(&key) {
        println!("Database has version information");
    }
    
    match db.delete(&String::from("version")) {
        Ok(_) => println!("Version information deleted"),
        Err(e)  => println!("Error: {}", e),
    }
    
    println!("Keys in database: {:?}", db.keys());
    println!("Number of entries: {}", db.len());
    
}

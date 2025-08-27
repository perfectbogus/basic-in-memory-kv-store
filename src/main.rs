use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

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

// Add explicit trait bounds to the struct definition
#[derive(Serialize, Deserialize)]
struct KeyValueStore<K, V>
where
    K: Eq + std::hash::Hash,
    V: Serialize + for<'de> Deserialize<'de>,
{
    data: HashMap<K, V>,
}

impl<K, V> KeyValueStore<K, V>
where
    K: Eq + std::hash::Hash + Clone + Serialize + for<'de> Deserialize<'de>,
    V: Clone + Serialize + for<'de> Deserialize<'de>,
{
    fn new() -> Self {
        KeyValueStore {
            data: HashMap::new(),
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
        if self.data.remove(key).is_some() {
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

    fn to_json(&self) -> DbResult<String> {
        serde_json::to_string(&self.data)
            .map_err(|e| DbError::SerializationError(e.to_string()))
    }

    fn from_json(json: &str) -> DbResult<Self> {
        let data = serde_json::from_str(json)
            .map_err(|e| DbError::SerializationError(e.to_string()))?;
        Ok(KeyValueStore { data })
    }

    fn save_to_file(&self, path: &Path) -> DbResult<()> {
        let json = self.to_json()?;

        let mut file = File::create(path)
            .map_err(|e| DbError::StorageError(e.to_string()))?;

        file.write_all(json.as_bytes())
            .map_err(|e| DbError::StorageError(e.to_string()))?;

        Ok(())
    }

    fn load_from_file(path: &Path) -> DbResult<Self> {
        let mut file = File::open(path)
            .map_err(|e| DbError::StorageError(e.to_string()))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| DbError::StorageError(e.to_string()))?;

        Self::from_json(&contents)
    }
}

fn main() {
    // Create a new database instance
    let mut db = KeyValueStore::<String, String>::new();

    // Insert some data
    db.set(String::from("name"), String::from("Rust Database")).unwrap();
    db.set(String::from("version"), String::from("0.1.0")).unwrap();
    db.set(String::from("author"), String::from("Rust Enthusiast")).unwrap();

    // Save database to a file
    let db_path = Path::new("database.json");
    db.save_to_file(db_path).unwrap();
    println!("Database saved to {}", db_path.display());

    // Load database from file
    let loaded_db = KeyValueStore::<String, String>::load_from_file(db_path).unwrap();
    println!("Database loaded successfully");
    println!("Database contains {} entries", loaded_db.len());

    // Print all keys
    println!("Keys in database: {:?}", loaded_db.keys());

    // Access some data
    println!("Database name: {}", loaded_db.get(&String::from("name")).unwrap());
}
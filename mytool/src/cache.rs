use anyhow::{Result, Context};
use rocksdb::{DB, Options};
use serde::{de::DeserializeOwned, Serialize};
use std::path::PathBuf;

pub struct Cache {
    db: DB,
}

impl Cache {
    pub fn new(path: PathBuf) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)
            .context("Failed to open RocksDB")?;
        Ok(Self { db })
    }

    pub fn put<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let serialized_value = serde_json::to_vec(value)
            .context("Failed to serialize value for cache")?;
        self.db.put(key, serialized_value)
            .context("Failed to put value into RocksDB")?;
        Ok(())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let Some(serialized_value) = self.db.get(key)
            .context("Failed to get value from RocksDB")? else {
                return Ok(None);
            };

        let value: T = serde_json::from_slice(&serialized_value)
            .context("Failed to deserialize value from cache")?;
        Ok(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir; // To create temporary directories for RocksDB
    use serde::Deserialize;

    #[test]
    fn test_cache_new_and_close() -> Result<()> {
        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let _cache = Cache::new(cache_path)?; // Just ensure it opens and closes without error
        Ok(())
    }

    #[test]
    fn test_cache_put_get_string() -> Result<()> {
        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cache = Cache::new(cache_path)?;

        let key = "test_string_key";
        let value = "test_string_value";

        cache.put(key, &value)?;
        let retrieved_value: Option<String> = cache.get(key)?;

        assert_eq!(retrieved_value, Some(value.to_string()));

        Ok(())
    }

    #[test]
    fn test_cache_get_non_existent() -> Result<()> {
        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cache = Cache::new(cache_path)?;

        let key = "non_existent_key";
        let retrieved_value: Option<String> = cache.get(key)?;

        assert_eq!(retrieved_value, None);

        Ok(())
    }

    #[test]
    fn test_cache_put_get_struct() -> Result<()> {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct TestData {
            name: String,
            id: u32,
            active: bool,
        }

        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cache = Cache::new(cache_path)?;

        let key = "test_struct_key";
        let value = TestData {
            name: "TestName".to_string(),
            id: 42,
            active: true,
        };

        cache.put(key, &value)?;
        let retrieved_value: Option<TestData> = cache.get(key)?;

        assert_eq!(retrieved_value, Some(value));

        Ok(())
    }

    #[test]
    fn test_cache_overwrite_key() -> Result<()> {
        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cache = Cache::new(cache_path)?;

        let key = "overwrite_key";
        let value1 = "first_value";
        let value2 = "second_value";

        cache.put(key, &value1)?;
        let retrieved1: Option<String> = cache.get(key)?;
        assert_eq!(retrieved1, Some(value1.to_string()));

        cache.put(key, &value2)?;
        let retrieved2: Option<String> = cache.get(key)?;
        assert_eq!(retrieved2, Some(value2.to_string()));

        Ok(())
    }

    #[test]
    fn test_cache_put_get_complex_struct() -> Result<()> {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct InnerData {
            items: Vec<String>,
            count: usize,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct ComplexData {
            id: String,
            inner: InnerData,
            enabled: bool,
        }

        let dir = tempdir()?;
        let cache_path = dir.path().to_path_buf();
        let cache = Cache::new(cache_path)?;

        let key = "complex_data_key";
        let value = ComplexData {
            id: "complex-1".to_string(),
            inner: InnerData {
                items: vec!["a".to_string(), "b".to_string()],
                count: 2,
            },
            enabled: true,
        };

        cache.put(key, &value)?;
        let retrieved_value: Option<ComplexData> = cache.get(key)?;

        assert_eq!(retrieved_value, Some(value));

        Ok(())
    }
}

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        let map = self
            .data
            .lock()
            .map_err(|e| anyhow!("{}", e.to_string()))?
            .clone();
        Ok(map)
    }

    pub fn increase(&self, key: impl Into<String>) -> Result<()> {
        let mut map = self.data.lock().map_err(|e| anyhow!("{}", e.to_string()))?;
        let value = map.entry(key.into()).or_insert(0);
        *value += 1;
        Ok(())
    }
}

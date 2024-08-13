use anyhow::Result;
use dashmap::DashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics {
    // data: Arc<RwLock<HashMap<String, i64>>>,
    data: Arc<DashMap<String, i64>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(DashMap::new()),
        }
    }

    // pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
    //     let map = self
    //         .data
    //         .read()
    //         .map_err(|e| anyhow!("{}", e.to_string()))?
    //         .clone();
    //     Ok(map)
    // }

    pub fn increase(&self, key: impl Into<String>) -> Result<()> {
        let mut value = self.data.entry(key.into()).or_insert(0);
        *value += 1;
        Ok(())
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}:{}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}

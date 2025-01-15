use serde_json::Value;
use std::{collections::HashMap, path::PathBuf};

use crate::AppResult;

pub fn init(file: PathBuf) -> AppResult<HashMap<String, Value>> {
    let settings = config::Config::builder()
        .add_source(config::File::from(file))
        .build()?;
    let result = settings.try_deserialize::<HashMap<String, Value>>()?;
    Ok(result)
}

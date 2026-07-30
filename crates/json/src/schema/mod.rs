use error::{CoreError, Result};
use serde_json::Value;

pub fn validate_json_schema(data: &Value, required_fields: &[&str]) -> Result<()> {
    for field in required_fields {
        if !data.get(field).map_or(false, |v| !v.is_null()) {
            return Err(CoreError::InvalidData(format!(
                "Missing required field: {}",
                field
            )));
        }
    }
    Ok(())
}

pub fn merge_json(base: &mut Value, overlay: &Value) {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            for (key, value) in overlay_map {
                if let Some(existing) = base_map.get_mut(key) {
                    merge_json(existing, value);
                } else {
                    base_map.insert(key.clone(), value.clone());
                }
            }
        }
        (base, overlay) => *base = overlay.clone(),
    }
}

pub fn deep_merge(base: &Value, overlay: &Value) -> Value {
    match (base, overlay) {
        (Value::Object(base_map), Value::Object(overlay_map)) => {
            let mut merged = base_map.clone();
            for (key, value) in overlay_map {
                if let Some(existing) = merged.get(key) {
                    merged.insert(key.clone(), deep_merge(existing, value));
                } else {
                    merged.insert(key.clone(), value.clone());
                }
            }
            Value::Object(merged)
        }
        _ => overlay.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_validate_required_fields() {
        let data = json!({"name": "test", "version": "1.0"});
        assert!(validate_json_schema(&data, &["name", "version"]).is_ok());
        assert!(validate_json_schema(&data, &["name", "missing"]).is_err());
    }

    #[test]
    fn test_merge_json() {
        let mut base = json!({"a": 1, "b": {"c": 2}});
        let overlay = json!({"b": {"d": 3}, "e": 4});
        merge_json(&mut base, &overlay);
        assert_eq!(base["a"], 1);
        assert_eq!(base["b"]["c"], 2);
        assert_eq!(base["b"]["d"], 3);
        assert_eq!(base["e"], 4);
    }

    #[test]
    fn test_deep_merge() {
        let base = json!({"a": {"b": 1}});
        let overlay = json!({"a": {"c": 2}});
        let merged = deep_merge(&base, &overlay);
        assert_eq!(merged["a"]["b"], 1);
        assert_eq!(merged["a"]["c"], 2);
    }
}

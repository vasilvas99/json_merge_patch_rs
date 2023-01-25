use serde_json::{Map, Value};

/// Implements  IETF-RFC7396
/// Modifies target in-place
/// The used unwraps are safe and can't panics due to the checks before them
pub fn merge_json(target: &mut Value, patch: &Value) {
    if patch.is_object() {
        if !target.is_object() {
            *target = Value::Object(Map::new());
        }

        let target_map = target.as_object_mut().unwrap();
        let patch_map = patch.as_object().unwrap();

        for (key, value) in patch_map {
            if value.is_null() {
                target_map.remove(key);
            } else {
                merge_json(target_map.entry(key).or_insert(Value::Null), value);
            }
        }
    } else {
        // Only the last leaf node is cloned. Still a copy though.
        *target = patch.clone();
    }
}

/// Not specified in the RFC, but a best effort attempt that would satisfy the property:
/// merge_json(source, create_patch(source, target)) == target
/// (merge_json is the left-inverse of create patch)
pub fn create_patch(source: Value, target: Value) -> Option<Value> {
    if !target.is_object() || !source.is_object() {
        return Some(target);
    }

    let mut result = Value::Object(Map::new());
    let unique_keys = source
        .as_object()?
        .keys()
        // This unwrap is safe, as whether target is a valid object is checked above
        .filter(|k| !target.as_object().unwrap().contains_key(*k));

    for key in unique_keys {
        result[key] = Value::Null;
    }

    for (key, value) in target.as_object()?.iter() {
        if !source.as_object().unwrap().contains_key(key) {
            result[key] = value.clone();
            continue;
        }
        if *value == source[key] {
            continue;
        }
        result[key] = create_patch(source[key].clone(), value.clone())?;
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use crate::*;
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref TEMPLATES_DIR: String =
            env!("CARGO_MANIFEST_DIR").to_string() + "/test_files/templates/";
        pub static ref PATCHES_DIR: String =
            env!("CARGO_MANIFEST_DIR").to_string() + "/test_files/patches/";
        pub static ref EXPECTED_DIR: String =
            env!("CARGO_MANIFEST_DIR").to_string() + "/test_files/expected/";
    }

    fn test_from_path(template_path: &str, patch_path: &str, expected_path: &str) {
        let template = std::fs::read_to_string(template_path).unwrap();
        let mut template: Value = serde_json::from_str(&template).unwrap();
        println!("{}", serde_json::to_string_pretty(&template).unwrap());
        let patch = std::fs::read_to_string(patch_path).unwrap();
        let patch: Value = serde_json::from_str(&patch).unwrap();

        let expected = std::fs::read_to_string(expected_path).unwrap();
        let expected: Value = serde_json::from_str(&expected).unwrap();

        merge_json(&mut template, &patch);
        println!("{}", serde_json::to_string_pretty(&template).unwrap());

        assert_eq!(template, expected);
    }

    #[test]
    fn create_patch_rfc_example() {
        let target = &(EXPECTED_DIR.clone() + "rfc_expected.json");
        let target = std::fs::read_to_string(target).unwrap();
        let target: Value = serde_json::from_str(&target).unwrap();

        let source = &(TEMPLATES_DIR.clone() + "rfc_template.json");
        let source = std::fs::read_to_string(source).unwrap();
        let source: Value = serde_json::from_str(&source).unwrap();

        let patch = &(PATCHES_DIR.clone() + "rfc_patch.json");
        let patch = std::fs::read_to_string(patch).unwrap();
        let patch: Value = serde_json::from_str(&patch).unwrap();

        assert_eq!(create_patch(source, target).unwrap(), patch)

    }
    #[test]
    fn rfc_example_test() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/rfc_template.json"),
            &(PATCHES_DIR.clone() + "rfc_patch.json"),
            &(EXPECTED_DIR.clone() + "rfc_expected.json"),
        );
    }

    #[test]
    fn cloudconnector_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "cloudconnector.json"),
            &(EXPECTED_DIR.clone() + "cloudconnector.json"),
        );
    }

    #[test]
    fn databroker_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "databroker.json"),
            &(EXPECTED_DIR.clone() + "databroker.json"),
        );
    }

    #[test]
    fn sua_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "sua.json"),
            &(EXPECTED_DIR.clone() + "sua.json"),
        );
    }

    #[test]
    fn vum_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "sua.json"),
            &(EXPECTED_DIR.clone() + "sua.json"),
        );
    }

    #[test]
    fn feedercan_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "feedercan.json"),
            &(EXPECTED_DIR.clone() + "feedercan.json"),
        );
    }

    #[test]
    fn otelcol_sdv_agent_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "otelcol-sdv-agent.json"),
            &(EXPECTED_DIR.clone() + "otelcol-sdv-agent.json"),
        );
    }

    #[test]
    fn otelcol_sdv_exporter_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "otelcol-sdv-exporter.json"),
            &(EXPECTED_DIR.clone() + "otelcol-sdv-exporter.json"),
        );
    }

    #[test]
    fn seatservice_json() {
        test_from_path(
            &(TEMPLATES_DIR.clone() + "/container_management.json"),
            &(PATCHES_DIR.clone() + "seatservice.json"),
            &(EXPECTED_DIR.clone() + "seatservice.json"),
        );
    }
}

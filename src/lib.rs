use anyhow::anyhow;
use serde_json::{Map, Value};

/// Implements  IETF-RFC7396
pub fn merge_json(target: &mut Value, patch: &Value) -> Result<(), Box<dyn std::error::Error>> {
    if patch.is_object() {
        if !target.is_object() {
            *target = Value::Object(Map::new());
        }

        let target_map = target
            .as_object_mut()
            .ok_or(anyhow!("Could not parse target as object"))?;
        let patch_obj = patch
            .as_object()
            .ok_or(anyhow!("Could not parse patch as object"))?;

        for (key, value) in patch_obj {
            if value.is_null() {
                target_map.remove(key);
            } else {
                merge_json(target_map.entry(key).or_insert(Value::Null), value)?;
            }
        }
    } else {
        // Only the last leaf node is cloned. Still a copy though.
        *target = patch.clone();
    }

    Ok(())
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

        merge_json(&mut template, &patch).unwrap();
        println!("{}", serde_json::to_string_pretty(&template).unwrap());

        assert_eq!(template, expected);
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

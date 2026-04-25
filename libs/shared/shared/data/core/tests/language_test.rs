use shared_shared_data_core::language::*;

#[test]
fn languages_default_is_en_us() {
    let lang = Languages::default();
    assert_eq!(lang.as_str(), "en-US");
}

#[test]
fn languages_as_str() {
    assert_eq!(Languages::EnUs.as_str(), "en-US");
    assert_eq!(Languages::ViVn.as_str(), "vi-VN");
}

#[test]
fn languages_as_bytes() {
    assert_eq!(Languages::EnUs.as_bytes(), b"en-US");
    assert_eq!(Languages::ViVn.as_bytes(), b"vi-VN");
}

#[test]
fn languages_serde_roundtrip() {
    let json = serde_json::to_string(&Languages::ViVn).unwrap();
    assert_eq!(json, "\"vi-VN\"");
    let parsed: Languages = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.as_str(), "vi-VN");

    let json = serde_json::to_string(&Languages::EnUs).unwrap();
    assert_eq!(json, "\"en-US\"");
    let parsed: Languages = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.as_str(), "en-US");
}

#[test]
fn extract_language_vi() {
    assert_eq!(extract_language("vi-VN").as_str(), "vi-VN");
    assert_eq!(extract_language("VI").as_str(), "vi-VN");
    assert_eq!(extract_language("vi").as_str(), "vi-VN");
}

#[test]
fn extract_language_defaults_to_en() {
    assert_eq!(extract_language("en-US").as_str(), "en-US");
    assert_eq!(extract_language("fr-FR").as_str(), "en-US");
    assert_eq!(extract_language("").as_str(), "en-US");
}

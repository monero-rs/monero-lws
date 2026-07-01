use serde_json::json;

use monero_lws::LoginResponse;

#[test]
fn test_deserialize_boolean() {
    let response_json = json!({
        "new_address": false,
        "generated_locally": true,
    });
    let response: LoginResponse = serde_json::from_value(response_json).unwrap();
    assert!(!response.new_address);
    assert!(response.generated_locally);
}

#[test]
fn test_deserialize_boolean_number() {
    let response_json = json!({
        "new_address": 0,
        "generated_locally": 1,
    });
    let response: LoginResponse = serde_json::from_value(response_json).unwrap();
    assert!(!response.new_address);
    assert!(response.generated_locally);
}

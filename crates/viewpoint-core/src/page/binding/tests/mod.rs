use super::*;

#[test]
fn test_binding_payload_deserialization() {
    let json = r#"{"seq": 1, "args": [1, 2, "hello"]}"#;
    let payload: BindingPayload = serde_json::from_str(json).unwrap();
    assert_eq!(payload.seq, 1);
    assert_eq!(payload.args.len(), 3);
    assert_eq!(payload.args[0], serde_json::json!(1));
    assert_eq!(payload.args[1], serde_json::json!(2));
    assert_eq!(payload.args[2], serde_json::json!("hello"));
}

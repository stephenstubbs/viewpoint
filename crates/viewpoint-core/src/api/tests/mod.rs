use super::*;

#[test]
fn test_http_method_to_reqwest() {
    assert_eq!(HttpMethod::Get.to_reqwest(), reqwest::Method::GET);
    assert_eq!(HttpMethod::Post.to_reqwest(), reqwest::Method::POST);
    assert_eq!(HttpMethod::Put.to_reqwest(), reqwest::Method::PUT);
    assert_eq!(HttpMethod::Patch.to_reqwest(), reqwest::Method::PATCH);
    assert_eq!(HttpMethod::Delete.to_reqwest(), reqwest::Method::DELETE);
    assert_eq!(HttpMethod::Head.to_reqwest(), reqwest::Method::HEAD);
}

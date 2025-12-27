use super::*;

#[test]
fn test_media_type_as_str() {
    assert_eq!(MediaType::Screen.as_str(), "screen");
    assert_eq!(MediaType::Print.as_str(), "print");
}

#[test]
fn test_vision_deficiency_default() {
    assert_eq!(VisionDeficiency::default(), VisionDeficiency::None);
}

#[test]
fn test_vision_deficiency_conversion() {
    let cdp: CdpVisionDeficiency = VisionDeficiency::Deuteranopia.into();
    assert_eq!(cdp, CdpVisionDeficiency::Deuteranopia);
}

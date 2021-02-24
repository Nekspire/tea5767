use tea5767::core;

mod common;

#[test]
fn test_tea67_new() {
    common::setup();
    let radio = core::new(i2c, 89.9, "EU/US", "stereo");
    assert_eq!(radio, {});

}
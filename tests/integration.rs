use tea5767::device;

mod common;

#[test]
fn test_tea67_new() {
    common::setup();
    let radio = device::new(i2c, 89.9, "EU/US", "stereo");
    assert_eq!(radio, {});

}
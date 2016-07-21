extern crate beam;
use beam::Beam;

#[test]
fn test_transcode_profiles() {
    let beam = Beam::new();
    let res = beam.transcodes.get_transcode_profiles();
    assert!(res.is_ok());
}

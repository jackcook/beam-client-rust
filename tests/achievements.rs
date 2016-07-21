extern crate beam;
use beam::Beam;

#[test]
fn test_achievements() {
    let beam = Beam::new();
    let res = beam.achievements.get_achievements();
    assert!(res.is_ok());
}

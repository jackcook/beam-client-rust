extern crate beam;
use beam::Beam;

#[test]
fn test_ingests() {
    let beam = Beam::new();
    let res = beam.ingests.get_ingests();
    assert!(res.is_ok());
}

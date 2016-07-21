extern crate beam;
use beam::Beam;

#[test]
fn test_ingests() {
    let beam = Beam::new();
    let res = beam.ingests.get_ingests();

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving ingests")
    }
}

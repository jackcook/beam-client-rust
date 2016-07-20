extern crate beam;

use beam::Beam;

#[test]
fn test_achievements() {
    let beam = Beam::new();
    let res = beam.achievements.get_achievements();

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving achievements")
    }
}

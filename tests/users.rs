extern crate beam;
use beam::Beam;

static USER_ID: u32 = 278;

#[test]
fn test_user_with_id() {
    let beam = Beam::new();
    let res = beam.users.get_user_with_id(USER_ID);
    assert!(res.is_ok());
}
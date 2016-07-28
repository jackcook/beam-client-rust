extern crate beam;
use beam::Beam;

static QUERY: &'static str = "jack";
static USER_ID: u32 = 278;

#[test]
fn test_user_with_id() {
    let beam = Beam::new();
    let res = beam.users.get_user_with_id(USER_ID);
    assert!(res.is_ok());
}

#[test]
fn test_users_by_query() {
    let beam = Beam::new();
    let res = beam.users.get_users_by_query(String::from(QUERY), 0);
    assert!(res.is_ok());
}

#[test]
fn test_achievements_of_user() {
    let beam = Beam::new();
    let res = beam.users.get_achievements_of_user(USER_ID);
    assert!(res.is_ok());
}

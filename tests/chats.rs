extern crate beam;

use beam::Beam;

static CHANNEL_ID: u32 = 3181;
static QUERY: &'static str = "aaaa";
static USER_ID: u32 = 3705;

#[test]
fn test_chat_details() {
    let beam = Beam::new();
    let res = beam.chats.get_chat_details(CHANNEL_ID);
    assert!(res.is_ok());
}

#[test]
fn test_chat_user() {
    let beam = Beam::new();
    let res = beam.chats.get_chat_user(CHANNEL_ID, USER_ID);
    assert!(res.is_ok());
}

#[test]
fn test_chat_users() {
    let beam = Beam::new();
    let res = beam.chats.get_chat_users(CHANNEL_ID, 0);
    assert!(res.is_ok());
}

#[test]
fn test_search_chat_users() {
    let beam = Beam::new();
    let res = beam.chats.search_chat_users(CHANNEL_ID, String::from(QUERY));
    assert!(res.is_ok());
}

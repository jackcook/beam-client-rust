extern crate beam;

use beam::Beam;
use beam::routes::channels::ChannelsRequestType;

static CHANNEL_ID: u32 = 3181;
static CHANNEL_TOKEN: &'static str = "jack";

#[test]
fn test_channel_with_id() {
    let beam = Beam::new();
    let res = beam.channels().get_channel_with_id(CHANNEL_ID);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channel")
    }
}

#[test]
fn test_channel_with_token() {
    let beam = Beam::new();
    let res = beam.channels().get_channel_with_token(String::from(CHANNEL_TOKEN));

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channel")
    }
}

#[test]
fn test_all_channels() {
    let beam = Beam::new();
    let res = beam.channels().get_channels(ChannelsRequestType::All, 0);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channels")
    }
}

#[test]
fn test_interactive_channels() {
    let beam = Beam::new();
    let res = beam.channels().get_channels(ChannelsRequestType::All, 0);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channels")
    }
}

#[test]
fn test_rising_channels() {
    let beam = Beam::new();
    let res = beam.channels().get_channels(ChannelsRequestType::Rising, 0);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channels")
    }
}

#[test]
fn test_fresh_channels() {
    let beam = Beam::new();
    let res = beam.channels().get_channels(ChannelsRequestType::Fresh, 0);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channels")
    }
}

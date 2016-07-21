extern crate beam;

use beam::Beam;
use beam::routes::channels::ChannelsRequestType;

static CHANNEL_ID: u32 = 3181;
static CHANNEL_TOKEN: &'static str = "jack";

#[test]
fn test_channel_with_id() {
    let beam = Beam::new();
    let res = beam.channels.get_channel_with_id(CHANNEL_ID);
    assert!(res.is_ok());
}

#[test]
fn test_channel_with_token() {
    let beam = Beam::new();
    let res = beam.channels.get_channel_with_token(String::from(CHANNEL_TOKEN));
    assert!(res.is_ok());
}

#[test]
fn test_all_channels() {
    let beam = Beam::new();
    let res = beam.channels.get_channels(ChannelsRequestType::All, 0);
    assert!(res.is_ok());
}

#[test]
fn test_interactive_channels() {
    let beam = Beam::new();
    let res = beam.channels.get_channels(ChannelsRequestType::All, 0);
    assert!(res.is_ok());
}

#[test]
fn test_rising_channels() {
    let beam = Beam::new();
    let res = beam.channels.get_channels(ChannelsRequestType::Rising, 0);
    assert!(res.is_ok());
}

#[test]
fn test_fresh_channels() {
    let beam = Beam::new();
    let res = beam.channels.get_channels(ChannelsRequestType::Fresh, 0);
    assert!(res.is_ok());
}

#[test]
fn test_followers_of_channel() {
    let beam = Beam::new();
    let res = beam.channels.get_followers_of_channel(CHANNEL_ID, 0);
    assert!(res.is_ok());
}

#[test]
fn test_emoticons_of_channel() {
    let beam = Beam::new();
    let res = beam.channels.get_emoticons_of_channel(CHANNEL_ID);
    assert!(res.is_ok());
}

#[test]
fn test_recordings_of_channel() {
    let beam = Beam::new();
    let res = beam.channels.get_recordings_of_channel(CHANNEL_ID);
    assert!(res.is_ok());
}

#[test]
fn test_viewers_of_channel() {
    let beam = Beam::new();
    let res = beam.channels.get_viewers_of_channel(CHANNEL_ID, 0);
    assert!(res.is_ok());
}

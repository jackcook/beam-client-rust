extern crate beam;
use beam::Beam;

static RECORDING_ID: u32 = 502;

#[test]
fn test_recording() {
    let beam = Beam::new();
    let res = beam.recordings.get_recording(RECORDING_ID);
    assert!(res.is_ok());
}

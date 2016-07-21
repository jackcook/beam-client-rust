extern crate beam;
use beam::Beam;

static RECORDING_ID: u32 = 420;

#[test]
fn test_recording() {
    let beam = Beam::new();
    let res = beam.recordings.get_recording(RECORDING_ID);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving recording")
    }
}

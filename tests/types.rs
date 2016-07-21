extern crate beam;
use beam::Beam;

static QUERY: &'static str = "Minecra";
static TYPE_ID: u32 = 127929;

#[test]
fn test_channels_by_type() {
    let beam = Beam::new();
    let res = beam.types.get_channels_by_type(TYPE_ID);
    assert!(res.is_ok());
}

#[test]
fn test_type() {
    let beam = Beam::new();
    let res = beam.types.get_type(TYPE_ID);
    assert!(res.is_ok());
}

#[test]
fn test_types() {
    let beam = Beam::new();
    let res = beam.types.get_types();
    assert!(res.is_ok());
}

#[test]
fn test_types_by_query() {
    let beam = Beam::new();
    let res = beam.types.get_types_by_query(String::from(QUERY));
    assert!(res.is_ok());
}

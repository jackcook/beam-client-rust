extern crate beam;
use beam::Beam;

static QUERY: &'static str = "Minecra";
static TYPE_ID: u32 = 127929;

#[test]
fn test_channels_by_type() {
    let beam = Beam::new();
    let res = beam.types.get_channels_by_type(TYPE_ID);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving channels")
    }
}

#[test]
fn test_type() {
    let beam = Beam::new();
    let res = beam.types.get_type(TYPE_ID);

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving type")
    }
}

#[test]
fn test_types() {
    let beam = Beam::new();
    let res = beam.types.get_types();

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving types")
    }
}

#[test]
fn test_types_by_query() {
    let beam = Beam::new();
    let res = beam.types.get_types_by_query(String::from(QUERY));

    match res {
        Ok(_) => assert!(true),
        Err(_) => panic!("error retrieving types")
    }
}

extern crate beam;
use beam::Beam;

static TEAM_ID: u32 = 111;
static TEAM_TOKEN: &'static str = "partners";

#[test]
fn test_team_with_id() {
    let beam = Beam::new();
    let res = beam.teams.get_team_with_id(TEAM_ID);
    assert!(res.is_ok());
}

#[test]
fn test_team_with_token() {
    let beam = Beam::new();
    let res = beam.teams.get_team_with_token(String::from(TEAM_TOKEN));
    assert!(res.is_ok());
}

#[test]
fn test_teams() {
    let beam = Beam::new();
    let res = beam.teams.get_teams();
    assert!(res.is_ok());
}

#[test]
fn test_members_of_team() {
    let beam = Beam::new();
    let res = beam.teams.get_members_of_team(TEAM_ID);
    assert!(res.is_ok());
}

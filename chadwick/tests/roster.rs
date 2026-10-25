use chadwick::{Player, Roster};

#[test]
fn test_roster_and_player_lifecycle() {
    let mut roster =
        Roster::new("BOS", 2023, "AL", "Boston", "Red Sox").expect("failed to create roster");

    let p1 =
        Player::new("willt001", "Williams", "Ted", 'L', 'R').expect("failed to create player");
    assert_eq!(p1.player_id(), Some("willt001"));
    assert_eq!(p1.last_name(), Some("Williams"));
    assert_eq!(p1.first_name(), Some("Ted"));
    assert_eq!(p1.bats(), 'L');
    assert_eq!(p1.throws(), 'R');

    let p2 =
        Player::new("ortid001", "Ortiz", "David", 'L', 'L').expect("failed to create player");

    // Append players to roster
    roster.append_player(p1);
    roster.append_player(p2);

    assert_eq!(roster.player_count(), 2);

    // Query player attributes from roster
    assert!(roster.has_player("willt001").unwrap());
    assert_eq!(roster.batting_hand("willt001").unwrap(), 'L');
    assert_eq!(roster.throwing_hand("willt001").unwrap(), 'R');

    assert!(roster.has_player("ortid001").unwrap());
    assert_eq!(roster.batting_hand("ortid001").unwrap(), 'L');
    assert_eq!(roster.throwing_hand("ortid001").unwrap(), 'L');

    assert!(!roster.has_player("unknown01").unwrap());
}

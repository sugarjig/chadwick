use chadwick::event_types::*;
use chadwick::ParsedEvent;

#[test]
fn test_parse_hits() {
    // Single
    let event = ParsedEvent::parse("S8").expect("failed to parse single");
    assert_eq!(event.event_type(), SINGLE);
    assert_eq!(event.runs_on_play(), 0);
    assert_eq!(event.rbi_on_play(), 0);
    assert_eq!(event.outs_on_play(), 0);
    assert!(event.is_official_ab());
    assert!(event.is_batter());

    // Double with runner advance and RBI
    let event = ParsedEvent::parse("D7/78.1-H;2-H").expect("failed to parse double");
    assert_eq!(event.event_type(), DOUBLE);
    assert_eq!(event.runs_on_play(), 2);
    assert_eq!(event.rbi_on_play(), 2);
    assert_eq!(event.outs_on_play(), 0);

    // Triple
    let event = ParsedEvent::parse("T9/F9D.1-H;2-H;3-H").expect("failed to parse triple");
    assert_eq!(event.event_type(), TRIPLE);
    assert_eq!(event.runs_on_play(), 3);
    assert_eq!(event.rbi_on_play(), 3);

    // Home Run
    let event = ParsedEvent::parse("HR/9/F9M.1-H").expect("failed to parse homerun");
    assert_eq!(event.event_type(), HOMERUN);
    assert_eq!(event.runs_on_play(), 2);
    assert_eq!(event.rbi_on_play(), 2);
}

#[test]
fn test_parse_outs_and_strikeouts() {
    // Strikeout swinging
    let event = ParsedEvent::parse("K").expect("failed to parse strikeout");
    assert_eq!(event.event_type(), STRIKEOUT);
    assert_eq!(event.outs_on_play(), 1);
    assert!(event.is_official_ab());

    // Ground out 6-3
    let event = ParsedEvent::parse("63/G6").expect("failed to parse ground out");
    assert_eq!(event.event_type(), GENERICOUT);
    assert_eq!(event.outs_on_play(), 1);
    assert_eq!(event.num_putouts(), 1);
    assert_eq!(event.putouts(), &[3]);
    assert_eq!(event.num_assists(), 1);
    assert_eq!(event.assists(), &[6]);

    // Fly out to center
    let event = ParsedEvent::parse("8/F8").expect("failed to parse fly out");
    assert_eq!(event.event_type(), GENERICOUT);
    assert_eq!(event.outs_on_play(), 1);
    assert_eq!(event.putouts(), &[8]);

    // Ground into double play (6-4-3)
    let event = ParsedEvent::parse("64(1)3/GDP/G6").expect("failed to parse double play");
    assert_eq!(event.event_type(), GENERICOUT);
    assert_eq!(event.outs_on_play(), 2);
    assert!(event.is_gdp());
    assert!(event.is_dp());
}

#[test]
fn test_parse_walks_and_hit_by_pitch() {
    // Walk
    let event = ParsedEvent::parse("W").expect("failed to parse walk");
    assert_eq!(event.event_type(), WALK);
    assert!(!event.is_official_ab());

    // Intentional Walk
    let event = ParsedEvent::parse("IW").expect("failed to parse intentional walk");
    assert_eq!(event.event_type(), INTENTIONALWALK);
    assert!(!event.is_official_ab());

    // Hit by pitch with bases loaded (RBI)
    let event = ParsedEvent::parse("HP.3-H;2-3;1-2").expect("failed to parse hit by pitch");
    assert_eq!(event.event_type(), HITBYPITCH);
    assert_eq!(event.runs_on_play(), 1);
    assert_eq!(event.rbi_on_play(), 1);
    assert!(!event.is_official_ab());
}

#[test]
fn test_parse_baserunning_and_steals() {
    // Stolen base
    let event = ParsedEvent::parse("SB2").expect("failed to parse stolen base");
    assert_eq!(event.event_type(), STOLENBASE);
    assert!(!event.is_batter());

    // Caught stealing
    let event = ParsedEvent::parse("CS2(24)").expect("failed to parse caught stealing");
    assert_eq!(event.event_type(), CAUGHTSTEALING);
    assert_eq!(event.outs_on_play(), 1);

    // Wild pitch and Passed ball
    let event = ParsedEvent::parse("WP.1-2").expect("failed to parse wild pitch");
    assert_eq!(event.event_type(), WILDPITCH);

    let event = ParsedEvent::parse("PB.2-3").expect("failed to parse passed ball");
    assert_eq!(event.event_type(), PASSEDBALL);

    // Defensive indifference
    let event = ParsedEvent::parse("DI.1-2").expect("failed to parse defensive indifference");
    assert_eq!(event.event_type(), INDIFFERENCE);
}

#[test]
fn test_parse_sacrifices_and_errors() {
    // Sac Fly
    let event = ParsedEvent::parse("9/SF.3-H").expect("failed to parse sac fly");
    assert_eq!(event.event_type(), GENERICOUT);
    assert!(event.is_sf());
    assert_eq!(event.runs_on_play(), 1);
    assert_eq!(event.rbi_on_play(), 1);
    assert!(!event.is_official_ab());

    // Sac Bunt
    let event = ParsedEvent::parse("14/SH.1-2").expect("failed to parse sac bunt");
    assert_eq!(event.event_type(), GENERICOUT);
    assert!(event.is_sh());
    assert!(!event.is_official_ab());

    // Fielding error
    let event = ParsedEvent::parse("E6/G6").expect("failed to parse fielding error");
    assert_eq!(event.event_type(), ERROR);
    assert_eq!(event.outs_on_play(), 0);
    assert!(event.is_official_ab());
}

use chadwick::{Game, GameFileReader, GameIterator};

#[test]
fn test_game_creation_and_attributes() {
    let mut game = Game::new(Some("BOS202304010")).expect("failed to create game");
    assert_eq!(game.game_id(), Some("BOS202304010"));

    // Add info fields
    game.append_info("visteam", "BAL").unwrap();
    game.append_info("hometeam", "BOS").unwrap();
    game.append_info("date", "2023/04/01").unwrap();

    assert_eq!(game.lookup_info("visteam").unwrap(), Some("BAL"));
    assert_eq!(game.lookup_info("hometeam").unwrap(), Some("BOS"));
    assert_eq!(game.lookup_info("date").unwrap(), Some("2023/04/01"));
    assert_eq!(game.lookup_info("nonexistent").unwrap(), None);

    // Add starters
    game.append_starter("mullc001", "Cedric Mullins", 0, 1, 8).unwrap();
    game.append_starter("verda001", "Alex Verdugo", 1, 1, 9).unwrap();

    assert!(game.has_info());
    assert!(game.has_starters());
}

#[test]
fn test_game_play_iteration_and_state() {
    let retrosheet_game = "id,BOS202304010\n\
version,1\n\
info,visteam,BAL\n\
info,hometeam,BOS\n\
info,date,2023/04/01\n\
start,mullc001,\"Cedric Mullins\",0,1,8\n\
start,rutsa001,\"Adley Rutschman\",0,2,2\n\
start,santc002,\"Anthony Santander\",0,3,9\n\
start,mounr001,\"Ryan Mountcastle\",0,4,3\n\
start,gunnh001,\"Gunnar Henderson\",0,5,5\n\
start,urair001,\"Ramon Urias\",0,6,4\n\
start,fraza001,\"Adam Frazier\",0,7,10\n\
start,hayea001,\"Austin Hays\",0,8,7\n\
start,maten001,\"Nick Maton\",0,9,6\n\
start,gibsk001,\"Kyle Gibson\",0,0,1\n\
start,verda001,\"Alex Verdugo\",1,1,9\n\
start,devev001,\"Rafael Devers\",1,2,5\n\
start,turnj001,\"Justin Turner\",1,3,10\n\
start,yosim001,\"Masataka Yoshida\",1,4,7\n\
start,duvaj001,\"Adam Duvall\",1,5,8\n\
start,casat001,\"Triston Casas\",1,6,3\n\
start,herne002,\"Enrique Hernandez\",1,7,6\n\
start,tapir001,\"Raimel Tapia\",1,8,4\n\
start,mcgre001,\"Reese McGuire\",1,9,2\n\
start,klucc001,\"Corey Kluber\",1,0,1\n\
play,1,0,mullc001,00,X,S8\n\
play,1,0,rutsa001,11,CBX,HR/9/F9M.1-H\n\
play,1,0,santc002,02,KSS,K\n\
play,1,0,mounr001,10,BX,43/G4\n\
play,1,0,gunnh001,22,BSCFS,K\n\
data,er,gibsk001,2\n";

    let mut reader = GameFileReader::from_str(retrosheet_game).expect("failed to open game file");
    let game = reader.read_next_game().expect("failed to read game");
    assert_eq!(game.game_id(), Some("BOS202304010"));

    let mut iter = GameIterator::new(&game).expect("failed to create game iterator");

    // Event 1: mullc001 singles to center (State entering Event 1)
    assert!(iter.is_parse_ok());
    assert_eq!(iter.lineup_slot(0, "mullc001").unwrap(), 1);
    assert_eq!(iter.player_position(0, "mullc001").unwrap(), 8);
    assert_eq!(iter.event_count(), Some(0));
    assert_eq!(iter.outs(), Some(0));
    assert_eq!(iter.score(0), Some(0));
    assert!(!iter.is_base_occupied(1));

    // Advance to Event 2: rutsa001 hits 2-run HR (State entering Event 2: runner on 1st)
    iter.next();
    assert!(iter.is_parse_ok());
    assert_eq!(iter.event_count(), Some(1));
    assert_eq!(iter.outs(), Some(0));
    assert_eq!(iter.score(0), Some(0));
    assert!(iter.is_base_occupied(1)); // Mullins is on 1st

    // Advance to Event 3: santc002 strikeout (State entering Event 3: 2-0 score, bases empty)
    iter.next();
    assert!(iter.is_parse_ok());
    assert_eq!(iter.event_count(), Some(2));
    assert_eq!(iter.outs(), Some(0));
    assert_eq!(iter.score(0), Some(2));
    assert!(!iter.is_base_occupied(1));

    // Advance to Event 4: mounr001 grounds out 4-3 (State entering Event 4: 1 out)
    iter.next();
    assert!(iter.is_parse_ok());
    assert_eq!(iter.event_count(), Some(3));
    assert_eq!(iter.outs(), Some(1));
    assert_eq!(iter.score(0), Some(2));

    // Advance to Event 5: gunnh001 strikeouts (State entering Event 5: 2 outs)
    iter.next();
    assert!(iter.is_parse_ok());
    assert_eq!(iter.event_count(), Some(4));
    assert_eq!(iter.outs(), Some(2));
    assert_eq!(iter.score(0), Some(2));

    // Test gameiter reset
    iter.reset();
    assert_eq!(iter.event_count(), Some(0));
    assert_eq!(iter.score(0), Some(0));
    assert_eq!(iter.outs(), Some(0));
}

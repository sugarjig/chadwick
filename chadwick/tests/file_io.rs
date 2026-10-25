use chadwick::{parse_int, GameFileReader, RecordReader, Scorebook, Tokenizer};

#[test]
fn test_record_reader_and_tokenizer() {
    let content = "id,BOS202304010\n\
version,1\n\
info,visteam,BAL\n";

    let mut reader = RecordReader::from_str(content).expect("failed to open record reader");

    // Read line 1
    let line1 = reader.next_line().expect("expected line 1");
    assert_eq!(line1, "id,BOS202304010");

    // Tokenize line 1
    let mut tokenizer = Tokenizer::new(line1).expect("failed to create tokenizer");
    assert_eq!(tokenizer.next_token(), Some("id"));
    assert_eq!(tokenizer.next_token(), Some("BOS202304010"));
    assert_eq!(tokenizer.next_token(), None);

    // Read line 2
    let line2 = reader.next_line().expect("expected line 2");
    assert_eq!(line2, "version,1");

    // Read line 3
    let line3 = reader.next_line().expect("expected line 3");
    assert_eq!(line3, "info,visteam,BAL");

    // EOF
    assert_eq!(reader.next_line(), None);
}

#[test]
fn test_file_find_game() {
    let multi_game_data = "id,BAL202304010\n\
version,1\n\
info,visteam,BOS\n\
id,BOS202304020\n\
version,1\n\
info,visteam,BAL\n";

    let mut reader = GameFileReader::from_str(multi_game_data).expect("failed to open game file");

    let game = reader
        .find_game("BOS202304020")
        .unwrap()
        .expect("game not found");
    assert_eq!(game.game_id(), Some("BOS202304020"));

    let first_game = reader
        .find_first_game()
        .unwrap()
        .expect("first game not found");
    assert_eq!(first_game.game_id(), Some("BAL202304010"));
}

#[test]
fn test_cw_atoi_and_scorebook() {
    assert_eq!(parse_int("1234").unwrap(), 1234);

    let book = Scorebook::new().expect("failed to create scorebook");
    assert!(!book.as_raw().is_null());
}

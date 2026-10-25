use chadwick_sys::*;
use std::ffi::{CStr, CString};

#[test]
fn test_ffi_game_creation_and_attributes() {
    unsafe {
        let game_id = CString::new("BOS202304010").unwrap();
        let game = cw_game_create(game_id.as_ptr() as *mut _);
        assert!(!game.is_null());

        let vteam_lbl = CString::new("visteam").unwrap();
        let vteam_val = CString::new("BAL").unwrap();
        cw_game_info_append(game, vteam_lbl.as_ptr() as *mut _, vteam_val.as_ptr() as *mut _);

        let hteam_lbl = CString::new("hometeam").unwrap();
        let hteam_val = CString::new("BOS").unwrap();
        cw_game_info_append(game, hteam_lbl.as_ptr() as *mut _, hteam_val.as_ptr() as *mut _);

        let date_lbl = CString::new("date").unwrap();
        let date_val = CString::new("2023/04/01").unwrap();
        cw_game_info_append(game, date_lbl.as_ptr() as *mut _, date_val.as_ptr() as *mut _);

        let p1_id = CString::new("mullc001").unwrap();
        let p1_name = CString::new("Cedric Mullins").unwrap();
        cw_game_starter_append(
            game,
            p1_id.as_ptr() as *mut _,
            p1_name.as_ptr() as *mut _,
            0,
            1,
            8,
        );

        let p2_id = CString::new("verda001").unwrap();
        let p2_name = CString::new("Alex Verdugo").unwrap();
        cw_game_starter_append(
            game,
            p2_id.as_ptr() as *mut _,
            p2_name.as_ptr() as *mut _,
            1,
            1,
            9,
        );

        assert!(!(*game).first_info.is_null());
        assert!(!(*game).first_starter.is_null());

        cw_game_cleanup(game);
    }
}

#[test]
fn test_ffi_game_play_iteration_and_state() {
    let retrosheet_game = b"id,BOS202304010\n\
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

    unsafe {
        let tmp = libc::tmpfile();
        assert!(!tmp.is_null());
        assert_eq!(
            libc::fwrite(
                retrosheet_game.as_ptr() as *const libc::c_void,
                1,
                retrosheet_game.len(),
                tmp
            ),
            retrosheet_game.len()
        );
        libc::rewind(tmp);

        let game = cw_game_read(tmp as *mut FILE);
        assert!(!game.is_null());

        let iter = cw_gameiter_create(game);
        assert!(!iter.is_null());

        assert_ne!((*iter).parse_ok, 0);
        let mullins_id = CString::new("mullc001").unwrap();
        assert_eq!(
            cw_gamestate_lineup_slot((*iter).state, 0, mullins_id.as_ptr() as *mut _),
            1
        );
        assert_eq!(
            cw_gamestate_player_position((*iter).state, 0, mullins_id.as_ptr() as *mut _),
            8
        );
        assert_eq!((*(*iter).state).event_count, 0);
        assert_eq!((*(*iter).state).outs, 0);
        assert_eq!((*(*iter).state).score[0], 0);
        assert_eq!(cw_gamestate_base_occupied((*iter).state, 1), 0);

        cw_gameiter_next(iter);
        assert_ne!((*iter).parse_ok, 0);
        assert_eq!((*(*iter).state).event_count, 1);
        assert_eq!((*(*iter).state).outs, 0);
        assert_eq!((*(*iter).state).score[0], 0);
        assert_ne!(cw_gamestate_base_occupied((*iter).state, 1), 0);

        cw_gameiter_next(iter);
        assert_ne!((*iter).parse_ok, 0);
        assert_eq!((*(*iter).state).event_count, 2);
        assert_eq!((*(*iter).state).outs, 0);
        assert_eq!((*(*iter).state).score[0], 2);
        assert_eq!(cw_gamestate_base_occupied((*iter).state, 1), 0);

        cw_gameiter_next(iter);
        assert_ne!((*iter).parse_ok, 0);
        assert_eq!((*(*iter).state).event_count, 3);
        assert_eq!((*(*iter).state).outs, 1);
        assert_eq!((*(*iter).state).score[0], 2);

        cw_gameiter_next(iter);
        assert_ne!((*iter).parse_ok, 0);
        assert_eq!((*(*iter).state).event_count, 4);
        assert_eq!((*(*iter).state).outs, 2);
        assert_eq!((*(*iter).state).score[0], 2);

        cw_gameiter_reset(iter);
        assert_eq!((*(*iter).state).event_count, 0);
        assert_eq!((*(*iter).state).score[0], 0);
        assert_eq!((*(*iter).state).outs, 0);

        cw_gameiter_cleanup(iter);
        libc::free(iter as *mut libc::c_void);

        cw_game_cleanup(game);
        libc::fclose(tmp);
    }
}

#[test]
fn test_ffi_roster_and_player() {
    unsafe {
        let team_id = CString::new("BOS").unwrap();
        let league = CString::new("AL").unwrap();
        let city = CString::new("Boston").unwrap();
        let nickname = CString::new("Red Sox").unwrap();
        let roster = cw_roster_create(
            team_id.as_ptr() as *mut _,
            2023,
            league.as_ptr() as *mut _,
            city.as_ptr() as *mut _,
            nickname.as_ptr() as *mut _,
        );
        assert!(!roster.is_null());

        let p1_id = CString::new("willt001").unwrap();
        let p1_last = CString::new("Williams").unwrap();
        let p1_first = CString::new("Ted").unwrap();
        let p1 = cw_player_create(
            p1_id.as_ptr() as *mut _,
            p1_last.as_ptr() as *mut _,
            p1_first.as_ptr() as *mut _,
            b'L' as i8,
            b'R' as i8,
        );
        assert!(!p1.is_null());

        let p2_id = CString::new("ortid001").unwrap();
        let p2_last = CString::new("Ortiz").unwrap();
        let p2_first = CString::new("David").unwrap();
        let p2 = cw_player_create(
            p2_id.as_ptr() as *mut _,
            p2_last.as_ptr() as *mut _,
            p2_first.as_ptr() as *mut _,
            b'L' as i8,
            b'L' as i8,
        );
        assert!(!p2.is_null());

        cw_roster_player_append(roster, p1);
        cw_roster_player_append(roster, p2);

        assert_eq!(cw_roster_player_count(roster), 2);

        let found = cw_roster_player_find(roster, p1_id.as_ptr() as *mut _);
        assert!(!found.is_null());
        assert_eq!(
            cw_roster_batting_hand(roster, p1_id.as_ptr() as *mut _) as u8,
            b'L'
        );
        assert_eq!(
            cw_roster_throwing_hand(roster, p1_id.as_ptr() as *mut _) as u8,
            b'R'
        );

        let not_found_id = CString::new("unknown01").unwrap();
        let not_found = cw_roster_player_find(roster, not_found_id.as_ptr() as *mut _);
        assert!(not_found.is_null());

        cw_roster_cleanup(roster);
    }
}

#[test]
fn test_ffi_record_reader_and_tokenizer() {
    let content = b"id,BOS202304010\n\
version,1\n\
info,visteam,BAL\n";

    unsafe {
        let tmp = libc::tmpfile();
        assert!(!tmp.is_null());
        assert_eq!(
            libc::fwrite(
                content.as_ptr() as *const libc::c_void,
                1,
                content.len(),
                tmp
            ),
            content.len()
        );
        libc::rewind(tmp);

        let mut reader: CWRecordReader = std::mem::zeroed();
        cw_record_reader_init(&mut reader, tmp as *mut FILE);

        assert_eq!(cw_record_reader_next(&mut reader), 1);
        let line1_ptr = cw_record_reader_line(&reader);
        assert!(!line1_ptr.is_null());
        let line1 = CStr::from_ptr(line1_ptr).to_str().unwrap();
        assert_eq!(line1, "id,BOS202304010");

        let mut line_buf = line1.as_bytes().to_vec();
        line_buf.push(0);
        let mut tokenizer: CWTokenizer = std::mem::zeroed();
        cw_tokenizer_init(&mut tokenizer, line_buf.as_mut_ptr() as *mut _);

        let tok1 = cw_tokenizer_next(&mut tokenizer);
        assert!(!tok1.is_null());
        assert_eq!(CStr::from_ptr(tok1).to_str().unwrap(), "id");

        let tok2 = cw_tokenizer_next(&mut tokenizer);
        assert!(!tok2.is_null());
        assert_eq!(CStr::from_ptr(tok2).to_str().unwrap(), "BOS202304010");

        let tok3 = cw_tokenizer_next(&mut tokenizer);
        assert!(tok3.is_null());

        assert_eq!(cw_record_reader_next(&mut reader), 1);
        let line2_ptr = cw_record_reader_line(&reader);
        assert_eq!(CStr::from_ptr(line2_ptr).to_str().unwrap(), "version,1");

        assert_eq!(cw_record_reader_next(&mut reader), 1);
        let line3_ptr = cw_record_reader_line(&reader);
        assert_eq!(
            CStr::from_ptr(line3_ptr).to_str().unwrap(),
            "info,visteam,BAL"
        );

        assert_eq!(cw_record_reader_next(&mut reader), 0);

        cw_record_reader_cleanup(&mut reader);
        libc::fclose(tmp);
    }
}

#[test]
fn test_ffi_file_find_game() {
    let multi_game_data = b"id,BAL202304010\n\
version,1\n\
info,visteam,BOS\n\
id,BOS202304020\n\
version,1\n\
info,visteam,BAL\n";

    unsafe {
        let tmp = libc::tmpfile();
        assert!(!tmp.is_null());
        assert_eq!(
            libc::fwrite(
                multi_game_data.as_ptr() as *const libc::c_void,
                1,
                multi_game_data.len(),
                tmp
            ),
            multi_game_data.len()
        );
        libc::rewind(tmp);

        let target_id = CString::new("BOS202304020").unwrap();
        let found = cw_file_find_game(target_id.as_ptr() as *mut _, tmp as *mut FILE);
        assert_ne!(found, 0);

        let game = cw_game_read(tmp as *mut FILE);
        assert!(!game.is_null());
        assert_eq!(
            CStr::from_ptr((*game).game_id).to_str().unwrap(),
            "BOS202304020"
        );

        cw_game_cleanup(game);

        libc::rewind(tmp);
        assert_ne!(cw_file_find_first_game(tmp as *mut FILE), 0);
        let first_game = cw_game_read(tmp as *mut FILE);
        assert!(!first_game.is_null());
        assert_eq!(
            CStr::from_ptr((*first_game).game_id).to_str().unwrap(),
            "BAL202304010"
        );
        cw_game_cleanup(first_game);

        libc::fclose(tmp);
    }
}

#[test]
fn test_ffi_parse_event_types() {
    unsafe {
        let s = CString::new("HR/9/F9M.1-H").unwrap();
        let mut event: CWEventData = std::mem::zeroed();
        assert_ne!(cw_parse_event(s.as_ptr() as *mut _, &mut event), 0);
        assert_eq!(event.event_type, CWEventType_CW_EVENT_HOMERUN);
        assert_eq!(cw_event_runs_on_play(&mut event), 2);
        assert_eq!(cw_event_rbi_on_play(&mut event), 2);
        assert_eq!(cw_event_outs_on_play(&mut event), 0);
    }
}

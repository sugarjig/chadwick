//! Raw FFI bindings to the Chadwick C library.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub use libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn test_game_create_and_cleanup() {
        unsafe {
            let game_id = CString::new("BOS202304010").unwrap();
            let game = cw_game_create(game_id.as_ptr() as *mut _);
            assert!(!game.is_null());

            let label = CString::new("visteam").unwrap();
            let data = CString::new("BAL").unwrap();
            cw_game_info_append(game, label.as_ptr() as *mut _, data.as_ptr() as *mut _);

            let batter = CString::new("mullc001").unwrap();
            let count = CString::new("00").unwrap();
            let pitches = CString::new("B").unwrap();
            let play = CString::new("S8").unwrap();
            cw_game_event_append(
                game,
                1,
                0,
                batter.as_ptr() as *mut _,
                count.as_ptr() as *mut _,
                pitches.as_ptr() as *mut _,
                play.as_ptr() as *mut _,
            );

            assert!(!(*game).first_event.is_null());
            cw_game_cleanup(game);
        }
    }

    #[test]
    fn test_game_create_null_and_cleanup() {
        unsafe {
            let game = cw_game_create(ptr::null_mut());
            assert!(!game.is_null());
            cw_game_cleanup(game);
        }
    }

    #[test]
    fn test_game_read_and_iterate() {
        let game_data = b"id,BOS202304010\n\
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
data,er,gibsk001,2\n";

        unsafe {
            let tmp = libc::tmpfile();
            assert!(!tmp.is_null());
            assert_eq!(
                libc::fwrite(
                    game_data.as_ptr() as *const libc::c_void,
                    1,
                    game_data.len(),
                    tmp
                ),
                game_data.len()
            );
            libc::rewind(tmp);

            let game = cw_game_read(tmp as *mut FILE);
            assert!(!game.is_null());

            let iter = cw_gameiter_create(game);
            assert!(!iter.is_null());

            assert_ne!((*iter).parse_ok, 0); // Nonzero indicates successful parse
            cw_gameiter_next(iter);

            cw_gameiter_cleanup(iter);
            libc::free(iter as *mut libc::c_void);

            cw_game_cleanup(game);
            libc::fclose(tmp);
        }
    }

    #[test]
    fn test_scorebook_create_and_cleanup() {
        unsafe {
            let book = cw_scorebook_create();
            assert!(!book.is_null());
            cw_scorebook_cleanup(book);
        }
    }

    #[test]
    fn test_player_and_roster() {
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

            let player_id = CString::new("willt001").unwrap();
            let last_name = CString::new("Williams").unwrap();
            let first_name = CString::new("Ted").unwrap();
            let player = cw_player_create(
                player_id.as_ptr() as *mut _,
                last_name.as_ptr() as *mut _,
                first_name.as_ptr() as *mut _,
                b'L' as i8,
                b'R' as i8,
            );
            assert!(!player.is_null());

            cw_player_cleanup(player);
            cw_roster_cleanup(roster);
        }
    }

    #[test]
    fn test_parse_event() {
        unsafe {
            let play_str = CString::new("HR/9/F9M").unwrap();
            let mut event: CWEventData = std::mem::zeroed();
            let res = cw_parse_event(play_str.as_ptr() as *mut _, &mut event);
            assert_ne!(res, 0); // Nonzero indicates success
            assert_eq!(event.event_type, CWEventType_CW_EVENT_HOMERUN);
            assert_eq!(cw_event_runs_on_play(&mut event), 1);
            assert_eq!(cw_event_rbi_on_play(&mut event), 1);
        }
    }
}

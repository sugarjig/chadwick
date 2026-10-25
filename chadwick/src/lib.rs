//! Safe and idiomatic Rust wrapper for the Chadwick baseball library.

pub use chadwick_sys as ffi;

use std::ffi::{CStr, CString, NulError};
use std::fmt;

pub mod event_types {
    use super::ffi;
    pub const UNKNOWN: u32 = ffi::CWEventType_CW_EVENT_UNKNOWN;
    pub const NONE: u32 = ffi::CWEventType_CW_EVENT_NONE;
    pub const GENERICOUT: u32 = ffi::CWEventType_CW_EVENT_GENERICOUT;
    pub const STRIKEOUT: u32 = ffi::CWEventType_CW_EVENT_STRIKEOUT;
    pub const STOLENBASE: u32 = ffi::CWEventType_CW_EVENT_STOLENBASE;
    pub const INDIFFERENCE: u32 = ffi::CWEventType_CW_EVENT_INDIFFERENCE;
    pub const CAUGHTSTEALING: u32 = ffi::CWEventType_CW_EVENT_CAUGHTSTEALING;
    pub const PICKOFFERROR: u32 = ffi::CWEventType_CW_EVENT_PICKOFFERROR;
    pub const PICKOFF: u32 = ffi::CWEventType_CW_EVENT_PICKOFF;
    pub const WILDPITCH: u32 = ffi::CWEventType_CW_EVENT_WILDPITCH;
    pub const PASSEDBALL: u32 = ffi::CWEventType_CW_EVENT_PASSEDBALL;
    pub const BALK: u32 = ffi::CWEventType_CW_EVENT_BALK;
    pub const OTHERADVANCE: u32 = ffi::CWEventType_CW_EVENT_OTHERADVANCE;
    pub const FOULERROR: u32 = ffi::CWEventType_CW_EVENT_FOULERROR;
    pub const WALK: u32 = ffi::CWEventType_CW_EVENT_WALK;
    pub const INTENTIONALWALK: u32 = ffi::CWEventType_CW_EVENT_INTENTIONALWALK;
    pub const HITBYPITCH: u32 = ffi::CWEventType_CW_EVENT_HITBYPITCH;
    pub const INTERFERENCE: u32 = ffi::CWEventType_CW_EVENT_INTERFERENCE;
    pub const ERROR: u32 = ffi::CWEventType_CW_EVENT_ERROR;
    pub const FIELDERSCHOICE: u32 = ffi::CWEventType_CW_EVENT_FIELDERSCHOICE;
    pub const SINGLE: u32 = ffi::CWEventType_CW_EVENT_SINGLE;
    pub const DOUBLE: u32 = ffi::CWEventType_CW_EVENT_DOUBLE;
    pub const TRIPLE: u32 = ffi::CWEventType_CW_EVENT_TRIPLE;
    pub const HOMERUN: u32 = ffi::CWEventType_CW_EVENT_HOMERUN;
    pub const MISSINGPLAY: u32 = ffi::CWEventType_CW_EVENT_MISSINGPLAY;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ChadwickError {
    NulError(NulError),
    ParseError(String),
    NullPointer,
    IoError(String),
}

impl fmt::Display for ChadwickError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChadwickError::NulError(e) => write!(f, "Nul byte error: {}", e),
            ChadwickError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ChadwickError::NullPointer => write!(f, "Null pointer error"),
            ChadwickError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for ChadwickError {}

impl From<NulError> for ChadwickError {
    fn from(err: NulError) -> Self {
        ChadwickError::NulError(err)
    }
}

/// A parsed Chadwick baseball play event.
#[derive(Debug, Clone)]
pub struct ParsedEvent {
    raw: ffi::CWEventData,
}

impl ParsedEvent {
    /// Parses a Retrosheet play event string.
    pub fn parse(text: &str) -> Result<Self, ChadwickError> {
        let c_text = CString::new(text)?;
        let mut raw = unsafe { std::mem::zeroed() };
        let res = unsafe { ffi::cw_parse_event(c_text.as_ptr() as *mut _, &mut raw) };
        if res != 0 {
            Ok(Self { raw })
        } else {
            Err(ChadwickError::ParseError(format!(
                "failed to parse event text: {}",
                text
            )))
        }
    }

    pub fn event_type(&self) -> u32 {
        self.raw.event_type
    }

    pub fn is_batter(&self) -> bool {
        unsafe { ffi::cw_event_is_batter(&self.raw as *const _ as *mut _) != 0 }
    }

    pub fn is_official_ab(&self) -> bool {
        unsafe { ffi::cw_event_is_official_ab(&self.raw as *const _ as *mut _) != 0 }
    }

    pub fn outs_on_play(&self) -> i32 {
        unsafe { ffi::cw_event_outs_on_play(&self.raw as *const _ as *mut _) }
    }

    pub fn runs_on_play(&self) -> i32 {
        unsafe { ffi::cw_event_runs_on_play(&self.raw as *const _ as *mut _) }
    }

    pub fn rbi_on_play(&self) -> i32 {
        unsafe { ffi::cw_event_rbi_on_play(&self.raw as *const _ as *mut _) }
    }

    pub fn runner_put_out(&self, runner: i32) -> bool {
        unsafe { ffi::cw_event_runner_put_out(&self.raw as *const _ as *mut _, runner) != 0 }
    }

    pub fn is_gdp(&self) -> bool {
        self.raw.gdp_flag != 0
    }

    pub fn is_dp(&self) -> bool {
        self.raw.dp_flag != 0
    }

    pub fn is_sf(&self) -> bool {
        self.raw.sf_flag != 0
    }

    pub fn is_sh(&self) -> bool {
        self.raw.sh_flag != 0
    }

    pub fn num_putouts(&self) -> i32 {
        self.raw.num_putouts
    }

    pub fn putouts(&self) -> &[i32] {
        let count = (self.raw.num_putouts as usize).min(self.raw.putouts.len());
        &self.raw.putouts[..count]
    }

    pub fn num_assists(&self) -> i32 {
        self.raw.num_assists
    }

    pub fn assists(&self) -> &[i32] {
        let count = (self.raw.num_assists as usize).min(self.raw.assists.len());
        &self.raw.assists[..count]
    }

    pub fn as_raw(&self) -> &ffi::CWEventData {
        &self.raw
    }

    pub fn as_raw_mut(&mut self) -> &mut ffi::CWEventData {
        &mut self.raw
    }
}

/// A file reader for Retrosheet game files.
pub struct GameFileReader {
    file_ptr: *mut libc::FILE,
}

impl std::str::FromStr for GameFileReader {
    type Err = ChadwickError;

    /// Opens a Retrosheet file by writing string content into a temporary file.
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let bytes = content.as_bytes();
        unsafe {
            let tmp = libc::tmpfile();
            if tmp.is_null() {
                return Err(ChadwickError::IoError("failed to create temp file".into()));
            }
            let written = libc::fwrite(
                bytes.as_ptr() as *const libc::c_void,
                1,
                bytes.len(),
                tmp,
            );
            if written != bytes.len() {
                libc::fclose(tmp);
                return Err(ChadwickError::IoError("failed to write to temp file".into()));
            }
            libc::rewind(tmp);
            Ok(Self { file_ptr: tmp })
        }
    }
}

impl GameFileReader {
    /// Opens a Retrosheet file by writing string content into a temporary file.
    pub fn from_str(content: &str) -> Result<Self, ChadwickError> {
        <Self as std::str::FromStr>::from_str(content)
    }

    /// Reads the next game from the file.
    pub fn read_next_game(&mut self) -> Option<Game> {
        unsafe {
            let ptr = ffi::cw_game_read(self.file_ptr as *mut ffi::FILE);
            if ptr.is_null() {
                None
            } else {
                Some(Game { raw: ptr })
            }
        }
    }

    /// Finds and reads a specific game by ID in the file.
    pub fn find_game(&mut self, game_id: &str) -> Result<Option<Game>, ChadwickError> {
        let c_id = CString::new(game_id)?;
        unsafe {
            let found = ffi::cw_file_find_game(c_id.as_ptr() as *mut _, self.file_ptr as *mut ffi::FILE);
            if found != 0 {
                let ptr = ffi::cw_game_read(self.file_ptr as *mut ffi::FILE);
                if ptr.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(Game { raw: ptr }))
                }
            } else {
                Ok(None)
            }
        }
    }

    /// Finds and reads the first game in the file after rewinding.
    pub fn find_first_game(&mut self) -> Result<Option<Game>, ChadwickError> {
        unsafe {
            libc::rewind(self.file_ptr);
            let found = ffi::cw_file_find_first_game(self.file_ptr as *mut ffi::FILE);
            if found != 0 {
                let ptr = ffi::cw_game_read(self.file_ptr as *mut ffi::FILE);
                if ptr.is_null() {
                    Ok(None)
                } else {
                    Ok(Some(Game { raw: ptr }))
                }
            } else {
                Ok(None)
            }
        }
    }
}

impl Drop for GameFileReader {
    fn drop(&mut self) {
        if !self.file_ptr.is_null() {
            unsafe {
                libc::fclose(self.file_ptr);
            }
        }
    }
}

/// A baseball game containing metadata, lineups, and play-by-play events.
pub struct Game {
    raw: *mut ffi::CWGame,
}

impl Game {

    /// Creates a new game with an optional game ID.
    pub fn new(game_id: Option<&str>) -> Result<Self, ChadwickError> {
        let ptr = match game_id {
            Some(id) => {
                let c_id = CString::new(id)?;
                unsafe { ffi::cw_game_create(c_id.as_ptr() as *mut _) }
            }
            None => unsafe { ffi::cw_game_create(std::ptr::null_mut()) },
        };
        if ptr.is_null() {
            return Err(ChadwickError::NullPointer);
        }
        Ok(Self { raw: ptr })
    }

    pub fn game_id(&self) -> Option<&str> {
        unsafe {
            if (*self.raw).game_id.is_null() {
                None
            } else {
                CStr::from_ptr((*self.raw).game_id).to_str().ok()
            }
        }
    }

    pub fn version(&self) -> Option<&str> {
        unsafe {
            if (*self.raw).version.is_null() {
                None
            } else {
                CStr::from_ptr((*self.raw).version).to_str().ok()
            }
        }
    }

    pub fn set_version(&mut self, version: &str) -> Result<(), ChadwickError> {
        let c_version = CString::new(version)?;
        unsafe {
            ffi::cw_game_set_version(self.raw, c_version.as_ptr() as *mut _);
        }
        Ok(())
    }

    pub fn append_info(&mut self, label: &str, data: &str) -> Result<(), ChadwickError> {
        let c_label = CString::new(label)?;
        let c_data = CString::new(data)?;
        unsafe {
            let label_dup = libc::strdup(c_label.as_ptr());
            let data_dup = libc::strdup(c_data.as_ptr());
            ffi::cw_game_info_append(self.raw, label_dup, data_dup);
        }
        Ok(())
    }

    pub fn lookup_info(&self, label: &str) -> Result<Option<&str>, ChadwickError> {
        let c_label = CString::new(label)?;
        unsafe {
            let res = ffi::cw_game_info_lookup(self.raw, c_label.as_ptr() as *mut _);
            if res.is_null() {
                Ok(None)
            } else {
                Ok(CStr::from_ptr(res).to_str().ok())
            }
        }
    }

    pub fn has_info(&self) -> bool {
        unsafe { !(*self.raw).first_info.is_null() }
    }

    pub fn has_starters(&self) -> bool {
        unsafe { !(*self.raw).first_starter.is_null() }
    }

    pub fn append_starter(
        &mut self,
        player_id: &str,
        name: &str,
        team: i32,
        slot: i32,
        pos: i32,
    ) -> Result<(), ChadwickError> {
        let c_pid = CString::new(player_id)?;
        let c_name = CString::new(name)?;
        unsafe {
            let pid_dup = libc::strdup(c_pid.as_ptr());
            let name_dup = libc::strdup(c_name.as_ptr());
            ffi::cw_game_starter_append(self.raw, pid_dup, name_dup, team, slot, pos);
        }
        Ok(())
    }

    pub fn append_event(
        &mut self,
        inning: i32,
        batting_team: i32,
        batter: &str,
        count: &str,
        pitches: &str,
        event_text: &str,
    ) -> Result<(), ChadwickError> {
        let c_batter = CString::new(batter)?;
        let c_count = CString::new(count)?;
        let c_pitches = CString::new(pitches)?;
        let c_event = CString::new(event_text)?;
        unsafe {
            let batter_dup = libc::strdup(c_batter.as_ptr());
            let count_dup = libc::strdup(c_count.as_ptr());
            let pitches_dup = libc::strdup(c_pitches.as_ptr());
            let event_dup = libc::strdup(c_event.as_ptr());
            ffi::cw_game_event_append(
                self.raw,
                inning,
                batting_team,
                batter_dup,
                count_dup,
                pitches_dup,
                event_dup,
            );
        }
        Ok(())
    }

    pub fn as_raw(&self) -> *mut ffi::CWGame {
        self.raw
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::cw_game_cleanup(self.raw);
                libc::free(self.raw as *mut libc::c_void);
            }
        }
    }
}

/// A player record.
pub struct Player {
    raw: *mut ffi::CWPlayer,
    owned: bool,
}

impl Player {
    pub fn new(
        player_id: &str,
        last_name: &str,
        first_name: &str,
        bats: char,
        throws: char,
    ) -> Result<Self, ChadwickError> {
        let c_pid = CString::new(player_id)?;
        let c_last = CString::new(last_name)?;
        let c_first = CString::new(first_name)?;
        let ptr = unsafe {
            ffi::cw_player_create(
                c_pid.as_ptr() as *mut _,
                c_last.as_ptr() as *mut _,
                c_first.as_ptr() as *mut _,
                bats as i8,
                throws as i8,
            )
        };
        if ptr.is_null() {
            return Err(ChadwickError::NullPointer);
        }
        Ok(Self {
            raw: ptr,
            owned: true,
        })
    }

    pub fn player_id(&self) -> Option<&str> {
        unsafe {
            if (*self.raw).player_id.is_null() {
                None
            } else {
                CStr::from_ptr((*self.raw).player_id).to_str().ok()
            }
        }
    }

    pub fn last_name(&self) -> Option<&str> {
        unsafe {
            if (*self.raw).last_name.is_null() {
                None
            } else {
                CStr::from_ptr((*self.raw).last_name).to_str().ok()
            }
        }
    }

    pub fn first_name(&self) -> Option<&str> {
        unsafe {
            if (*self.raw).first_name.is_null() {
                None
            } else {
                CStr::from_ptr((*self.raw).first_name).to_str().ok()
            }
        }
    }

    pub fn bats(&self) -> char {
        unsafe { (*self.raw).bats as u8 as char }
    }

    pub fn throws(&self) -> char {
        unsafe { (*self.raw).throws as u8 as char }
    }

    pub fn as_raw(&self) -> *mut ffi::CWPlayer {
        self.raw
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        if self.owned && !self.raw.is_null() {
            unsafe {
                ffi::cw_player_cleanup(self.raw);
                libc::free(self.raw as *mut libc::c_void);
            }
        }
    }
}

/// A team roster.
pub struct Roster {
    raw: *mut ffi::CWRoster,
}

impl Roster {
    pub fn new(
        team_id: &str,
        year: i32,
        league: &str,
        city: &str,
        nickname: &str,
    ) -> Result<Self, ChadwickError> {
        let c_team = CString::new(team_id)?;
        let c_league = CString::new(league)?;
        let c_city = CString::new(city)?;
        let c_nick = CString::new(nickname)?;
        let ptr = unsafe {
            ffi::cw_roster_create(
                c_team.as_ptr() as *mut _,
                year,
                c_league.as_ptr() as *mut _,
                c_city.as_ptr() as *mut _,
                c_nick.as_ptr() as *mut _,
            )
        };
        if ptr.is_null() {
            return Err(ChadwickError::NullPointer);
        }
        Ok(Self { raw: ptr })
    }

    pub fn append_player(&mut self, mut player: Player) {
        player.owned = false;
        unsafe {
            ffi::cw_roster_player_append(self.raw, player.raw);
        }
    }

    pub fn player_count(&self) -> i32 {
        unsafe { ffi::cw_roster_player_count(self.raw) }
    }

    pub fn batting_hand(&self, player_id: &str) -> Result<char, ChadwickError> {
        let c_pid = CString::new(player_id)?;
        let b = unsafe { ffi::cw_roster_batting_hand(self.raw, c_pid.as_ptr() as *mut _) };
        Ok(b as u8 as char)
    }

    pub fn throwing_hand(&self, player_id: &str) -> Result<char, ChadwickError> {
        let c_pid = CString::new(player_id)?;
        let t = unsafe { ffi::cw_roster_throwing_hand(self.raw, c_pid.as_ptr() as *mut _) };
        Ok(t as u8 as char)
    }

    pub fn has_player(&self, player_id: &str) -> Result<bool, ChadwickError> {
        let c_pid = CString::new(player_id)?;
        unsafe {
            let found = ffi::cw_roster_player_find(self.raw, c_pid.as_ptr() as *mut _);
            Ok(!found.is_null())
        }
    }

    pub fn as_raw(&self) -> *mut ffi::CWRoster {
        self.raw
    }
}

impl Drop for Roster {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::cw_roster_cleanup(self.raw);
                libc::free(self.raw as *mut libc::c_void);
            }
        }
    }
}

/// An iterator through plays in a game, tracking state changes.
pub struct GameIterator<'a> {
    raw: *mut ffi::CWGameIterator,
    _phantom: std::marker::PhantomData<&'a Game>,
}

impl<'a> GameIterator<'a> {
    pub fn new(game: &'a Game) -> Result<Self, ChadwickError> {
        let ptr = unsafe { ffi::cw_gameiter_create(game.raw) };
        if ptr.is_null() {
            return Err(ChadwickError::NullPointer);
        }
        Ok(Self {
            raw: ptr,
            _phantom: std::marker::PhantomData,
        })
    }

    pub fn reset(&mut self) {
        unsafe { ffi::cw_gameiter_reset(self.raw) };
    }

    pub fn next(&mut self) {
        unsafe { ffi::cw_gameiter_next(self.raw) };
    }

    pub fn is_valid(&self) -> bool {
        unsafe { !(*self.raw).event.is_null() }
    }

    pub fn is_parse_ok(&self) -> bool {
        unsafe { (*self.raw).parse_ok != 0 }
    }

    pub fn event_count(&self) -> Option<i32> {
        unsafe {
            if (*self.raw).state.is_null() {
                None
            } else {
                Some((*(*self.raw).state).event_count)
            }
        }
    }

    pub fn inning(&self) -> Option<i32> {
        unsafe {
            if (*self.raw).state.is_null() {
                None
            } else {
                Some((*(*self.raw).state).inning)
            }
        }
    }

    pub fn outs(&self) -> Option<i32> {
        unsafe {
            if (*self.raw).state.is_null() {
                None
            } else {
                Some((*(*self.raw).state).outs)
            }
        }
    }

    pub fn score(&self, team: usize) -> Option<i32> {
        if team >= 2 {
            return None;
        }
        unsafe {
            if (*self.raw).state.is_null() {
                None
            } else {
                Some((*(*self.raw).state).score[team])
            }
        }
    }

    pub fn is_base_occupied(&self, base: i32) -> bool {
        unsafe {
            if (*self.raw).state.is_null() {
                false
            } else {
                ffi::cw_gamestate_base_occupied((*self.raw).state, base) != 0
            }
        }
    }

    pub fn lineup_slot(&self, team: i32, player_id: &str) -> Result<i32, ChadwickError> {
        let c_pid = CString::new(player_id)?;
        unsafe {
            if (*self.raw).state.is_null() {
                return Err(ChadwickError::NullPointer);
            }
            Ok(ffi::cw_gamestate_lineup_slot(
                (*self.raw).state,
                team,
                c_pid.as_ptr() as *mut _,
            ))
        }
    }

    pub fn player_position(&self, team: i32, player_id: &str) -> Result<i32, ChadwickError> {
        let c_pid = CString::new(player_id)?;
        unsafe {
            if (*self.raw).state.is_null() {
                return Err(ChadwickError::NullPointer);
            }
            Ok(ffi::cw_gamestate_player_position(
                (*self.raw).state,
                team,
                c_pid.as_ptr() as *mut _,
            ))
        }
    }

    pub fn as_raw(&self) -> *mut ffi::CWGameIterator {
        self.raw
    }
}

impl<'a> Drop for GameIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::cw_gameiter_cleanup(self.raw);
            libc::free(self.raw as *mut libc::c_void);
        }
    }
}

/// A reader for Retrosheet record lines.
pub struct RecordReader {
    raw: ffi::CWRecordReader,
    file_ptr: *mut libc::FILE,
}

impl std::str::FromStr for RecordReader {
    type Err = ChadwickError;

    fn from_str(content: &str) -> Result<Self, Self::Err> {
        Self::from_str(content)
    }
}

impl RecordReader {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(content: &str) -> Result<Self, ChadwickError> {
        let bytes = content.as_bytes();
        unsafe {
            let tmp = libc::tmpfile();
            if tmp.is_null() {
                return Err(ChadwickError::IoError("failed to create temp file".into()));
            }
            let written = libc::fwrite(
                bytes.as_ptr() as *const libc::c_void,
                1,
                bytes.len(),
                tmp,
            );
            if written != bytes.len() {
                libc::fclose(tmp);
                return Err(ChadwickError::IoError("failed to write to temp file".into()));
            }
            libc::rewind(tmp);
            let mut raw = std::mem::zeroed();
            ffi::cw_record_reader_init(&mut raw, tmp as *mut ffi::FILE);
            Ok(Self { raw, file_ptr: tmp })
        }
    }

    pub fn next_line(&mut self) -> Option<&str> {
        unsafe {
            if ffi::cw_record_reader_next(&mut self.raw) == 1 {
                let ptr = ffi::cw_record_reader_line(&self.raw);
                if ptr.is_null() {
                    None
                } else {
                    CStr::from_ptr(ptr).to_str().ok()
                }
            } else {
                None
            }
        }
    }
}

impl Drop for RecordReader {
    fn drop(&mut self) {
        unsafe {
            ffi::cw_record_reader_cleanup(&mut self.raw);
            if !self.file_ptr.is_null() {
                libc::fclose(self.file_ptr);
            }
        }
    }
}

/// A tokenizer for parsing comma-delimited Retrosheet records.
pub struct Tokenizer {
    raw: ffi::CWTokenizer,
    _buffer: Vec<u8>,
}

impl Tokenizer {
    pub fn new(text: &str) -> Result<Self, ChadwickError> {
        let mut buffer = text.as_bytes().to_vec();
        buffer.push(0);
        let mut raw = unsafe { std::mem::zeroed() };
        unsafe {
            ffi::cw_tokenizer_init(&mut raw, buffer.as_mut_ptr() as *mut _);
        }
        Ok(Self {
            raw,
            _buffer: buffer,
        })
    }

    pub fn next_token(&mut self) -> Option<&str> {
        unsafe {
            let tok = ffi::cw_tokenizer_next(&mut self.raw);
            if tok.is_null() {
                None
            } else {
                CStr::from_ptr(tok).to_str().ok()
            }
        }
    }
}

impl Iterator for Tokenizer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token().map(|s| s.to_string())
    }
}

/// A safe wrapper for the Chadwick scorebook.
pub struct Scorebook {
    raw: *mut ffi::CWScorebook,
}

impl Scorebook {
    pub fn new() -> Result<Self, ChadwickError> {
        let ptr = unsafe { ffi::cw_scorebook_create() };
        if ptr.is_null() {
            Err(ChadwickError::NullPointer)
        } else {
            Ok(Self { raw: ptr })
        }
    }

    pub fn as_raw(&self) -> *mut ffi::CWScorebook {
        self.raw
    }
}

impl Drop for Scorebook {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::cw_scorebook_cleanup(self.raw);
                libc::free(self.raw as *mut libc::c_void);
            }
        }
    }
}

/// Safely parses an integer string using Chadwick's numeric conversion.
pub fn parse_int(s: &str) -> Result<i32, ChadwickError> {
    let c_str = CString::new(s)?;
    let val = unsafe { ffi::cw_atoi(c_str.as_ptr() as *mut _, std::ptr::null_mut()) };
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_abstraction() {
        let mut game = Game::new(Some("BOS202304010")).expect("game creation failed");
        assert_eq!(game.game_id(), Some("BOS202304010"));

        game.append_info("visteam", "BAL").unwrap();
        game.append_info("hometeam", "BOS").unwrap();
        assert_eq!(game.lookup_info("visteam").unwrap(), Some("BAL"));
        assert_eq!(game.lookup_info("hometeam").unwrap(), Some("BOS"));
        assert_eq!(game.lookup_info("nonexistent").unwrap(), None);

        game.append_starter("mullc001", "Cedric Mullins", 0, 1, 8)
            .unwrap();
    }

    #[test]
    fn test_parsed_event_abstraction() {
        let event = ParsedEvent::parse("HR/9/F9M.1-H").expect("failed to parse event");
        assert_eq!(event.event_type(), ffi::CWEventType_CW_EVENT_HOMERUN);
        assert_eq!(event.runs_on_play(), 2);
        assert_eq!(event.rbi_on_play(), 2);
        assert_eq!(event.outs_on_play(), 0);
        assert!(event.is_official_ab());
        assert!(event.is_batter());

        let walk = ParsedEvent::parse("W").expect("failed to parse walk");
        assert_eq!(walk.event_type(), ffi::CWEventType_CW_EVENT_WALK);
        assert!(!walk.is_official_ab());
    }

    #[test]
    fn test_roster_and_player_abstraction() {
        let mut roster =
            Roster::new("BOS", 2023, "AL", "Boston", "Red Sox").expect("roster creation failed");
        let player = Player::new("willt001", "Williams", "Ted", 'L', 'R')
            .expect("player creation failed");
        roster.append_player(player);

        assert_eq!(roster.player_count(), 1);
        assert_eq!(roster.batting_hand("willt001").unwrap(), 'L');
        assert_eq!(roster.throwing_hand("willt001").unwrap(), 'R');
    }

    #[test]
    fn test_game_iterator_abstraction() {
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
play,1,0,rutsa001,11,CBX,HR/9/F9M.1-H\n";

        let mut reader =
            GameFileReader::from_str(retrosheet_game).expect("failed to load retrosheet game");
        let game = reader.read_next_game().expect("game not found");
        let mut iter = GameIterator::new(&game).expect("failed to create iterator");
        assert!(iter.is_valid());
        assert_eq!(iter.inning(), Some(1));
        assert_eq!(iter.outs(), Some(0));

        iter.next();
        assert!(iter.is_valid());
        assert_eq!(iter.score(0), Some(0));
    }
}

use chrono::DateTime;
use rs_fsrs as fsrs;
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct Parameters {
    pub request_retention: f64,
    pub maximum_interval: i32,
    pub w: [f64; 19],
    pub decay: f64,
    pub factor: f64,
    pub enable_short_term: bool,
    pub enable_fuzz: bool,
    pub seed: *const c_char,
}

fn to_raw<T>(value: T) -> *const T {
    Box::into_raw(Box::new(value))
}

impl From<Parameters> for fsrs::Parameters {
    fn from(value: Parameters) -> Self {
        Self {
            request_retention: value.request_retention,
            maximum_interval: value.maximum_interval,
            w: value.w,
            decay: value.decay,
            factor: value.factor,
            enable_short_term: value.enable_short_term,
            enable_fuzz: value.enable_fuzz,
            seed: if value.seed.is_null() {
                fsrs::Seed::Default
            } else {
                fsrs::Seed::String(
                    unsafe { CStr::from_ptr(value.seed) }
                        .to_str()
                        .unwrap()
                        .to_string(),
                )
            },
        }
    }
}

#[repr(C)]
pub struct Fsrs(*const fsrs::FSRS);

impl Fsrs {
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_new(p: Parameters) -> Self {
        Self(to_raw(fsrs::FSRS::new(p.into())))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_default() -> Self {
        Self(to_raw(fsrs::FSRS::default()))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_repeat_timestamp(
        fsrs: *const Self,
        card: *const Card,
        now: i64,
    ) -> RecordLog {
        RecordLog(to_raw(unsafe { &*(*fsrs).0 }.repeat(
            unsafe { &*(*card).0 }.clone(),
            DateTime::from_timestamp(now, 0).expect("err"),
        )))
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct Card(*const fsrs::Card);

impl Card {
    #[no_mangle]
    pub extern "C" fn fsrs_Card_new() -> Self {
        Self(to_raw(fsrs::Card::new()))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_due(&self) -> i64 {
        unsafe { &*(*self).0 }.due.timestamp()
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_stability(&self) -> f64 {
        unsafe { &*(*self).0 }.stability
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_difficulty(&self) -> f64 {
        unsafe { &*(*self).0 }.difficulty
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_elapsed_days(&self) -> i64 {
        unsafe { &*(*self).0 }.elapsed_days
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_scheduled_days(&self) -> i64 {
        unsafe { &*(*self).0 }.scheduled_days
    }

    #[no_mangle]
    pub extern "C" fn fsrs_Card_reps(&self) -> i32 {
        unsafe { &*(*self).0 }.reps
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_lapses(&self) -> i32 {
        unsafe { &*(*self).0 }.lapses
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_state(&self) -> State {
        unsafe { &*(*self).0 }.state.into()
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_last_review(&self) -> i64 {
        unsafe { &*(*self).0 }.last_review.timestamp()
    }
}

#[repr(C)]
pub struct RecordLog(*const fsrs::RecordLog);
impl RecordLog {
    #[no_mangle]
    pub extern "C" fn fsrs_ScheduledCards_get(&self, r: Rating) -> SchedulingInfo {
        SchedulingInfo(to_raw(
            unsafe { &*(*self).0 }.get(&r.into()).unwrap().clone(),
        ))
    }
}

#[repr(C)]
pub struct SchedulingInfo(*const fsrs::SchedulingInfo);

impl SchedulingInfo {
    #[no_mangle]
    pub extern "C" fn fsrs_SchedulingInfo_card(&self) -> Card {
        Card(to_raw(unsafe { &*(*self).0 }.card.clone()))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_SchedulingInfo_review_log(&self) -> ReviewLog {
        ReviewLog(to_raw(unsafe { &*(*self).0 }.review_log.clone()))
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub enum Rating {
    #[default]
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}
impl From<Rating> for fsrs::Rating {
    fn from(value: Rating) -> Self {
        use fsrs::Rating as r;
        use Rating::*;
        match value {
            Again => r::Again,
            Hard => r::Hard,
            Good => r::Good,
            Easy => r::Easy,
        }
    }
}
impl From<fsrs::Rating> for Rating {
    fn from(value: fsrs::Rating) -> Self {
        use fsrs::Rating::*;
        use Rating as r;
        match value {
            Again => r::Again,
            Hard => r::Hard,
            Good => r::Good,
            Easy => r::Easy,
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug, Default, Eq)]
pub enum State {
    #[default]
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}

impl From<State> for fsrs::State {
    fn from(value: State) -> Self {
        use fsrs::State as r;
        use State::*;
        match value {
            New => r::New,
            Learning => r::Learning,
            Review => r::Review,
            Relearning => r::Relearning,
        }
    }
}

impl From<fsrs::State> for State {
    fn from(value: fsrs::State) -> Self {
        use fsrs::State::*;
        use State as r;
        match value {
            New => r::New,
            Learning => r::Learning,
            Review => r::Review,
            Relearning => r::Relearning,
        }
    }
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ReviewLog(*const fsrs::ReviewLog);

impl ReviewLog {
    #[no_mangle]
    pub extern "C" fn fsrs_ReviewLog_rating(&self) -> Rating {
        unsafe { &*(*self).0 }.rating.into()
    }

    #[no_mangle]
    pub extern "C" fn fsrs_ReviewLog_elapsed_days(&self) -> i64 {
        unsafe { &*(*self).0 }.elapsed_days
    }

    #[no_mangle]
    pub extern "C" fn fsrs_ReviewLog_scheduled_days(&self) -> i64 {
        unsafe { &*(*self).0 }.scheduled_days
    }

    #[no_mangle]
    pub extern "C" fn fsrs_ReviewLog_state(&self) -> State {
        unsafe { &*(*self).0 }.state.into()
    }

    #[no_mangle]
    pub extern "C" fn fsrs_ReviewLog_reviewed_date(&self) -> i64 {
        unsafe { &*(*self).0 }.reviewed_date.timestamp()
    }
}

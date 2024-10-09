use chrono::{DateTime, Utc};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    pub request_retention: f64,
    pub maximum_interval: i32,
    pub w: [f64; 19],
    pub decay: f64,
    pub factor: f64,
    pub enable_short_term: bool,
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
        let fsrs_inner = unsafe { &*(*fsrs).0 };
        let card_inner = unsafe { &*(*card).0 };
        RecordLog(to_raw(fsrs_inner.repeat(
            card_inner.clone(),
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
        let c = unsafe { &*(*self).0 };
        c.due.timestamp()
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_stability(&self) -> f64 {
        let c = unsafe { &*(*self).0 };
        c.stability
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_difficulty(&self) -> f64 {
        let c = unsafe { &*(*self).0 };
        c.difficulty
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_elapsed_days(&self) -> i64 {
        let c = unsafe { &*(*self).0 };
        c.elapsed_days
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_scheduled_days(&self) -> i64 {
        let c = unsafe { &*(*self).0 };
        c.scheduled_days
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_reps(&self) -> i32 {
        let c = unsafe { &*(*self).0 };
        c.reps
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_lapses(&self) -> i32 {
        let c = unsafe { &*(*self).0 };
        c.lapses
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_state(&self) -> State {
        let c = unsafe { &*(*self).0 };
        c.state.into()
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Card_last_review(&self) -> i64 {
        let c = unsafe { &*(*self).0 };
        c.last_review.timestamp()
    }
}

#[repr(C)]
pub struct RecordLog(*const fsrs::RecordLog);
impl RecordLog {
    #[no_mangle]
    pub extern "C" fn fsrs_ScheduledCards_get(&self, r: Rating) -> SchedulingInfo {
        let s = unsafe { &*(*self).0 };
        SchedulingInfo(to_raw(s.get(&r.into()).unwrap().clone()))
    }
}

#[repr(C)]
pub struct SchedulingInfo(*const fsrs::SchedulingInfo);

impl SchedulingInfo {
    #[no_mangle]
    pub extern "C" fn fsrs_SchedulingInfo_card(&self) -> Card {
        let s = unsafe { &*(*self).0 };
        Card(to_raw(s.card.clone()))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_SchedulingInfo_review_log(&self) -> ReviewLog {
        let s = unsafe { &*(*self).0 };
        let log = s.review_log.clone();
        ReviewLog {
            elapsed_days: log.elapsed_days,
            scheduled_days: log.scheduled_days,
            reviewed_date_s: log.reviewed_date.timestamp(),
            rating: log.rating.into(),
            state: log.state.into(),
        }
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
#[derive(Debug, Clone, Default)]
pub struct ReviewLog {
    pub rating: Rating,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub state: State,
    pub reviewed_date_s: i64,
}

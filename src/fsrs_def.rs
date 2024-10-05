use chrono::DateTime;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Parameters {
    pub request_retention: f32,
    pub maximum_interval: i32,
    pub w: [f32; 19],
}

impl From<Parameters> for fsrs::Parameters {
    fn from(value: Parameters) -> Self {
        Self {
            request_retention: value.request_retention,
            maximum_interval: value.maximum_interval,
            w: value.w,
        }
    }
}

#[repr(C)]
pub struct Fsrs(*const fsrs::FSRS);

impl Fsrs {
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_new(p: Parameters) -> Self {
        Self(Box::into_raw(Box::new(fsrs::FSRS::new(p.into()))))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_default() -> Self {
        Self(Box::into_raw(Box::new(fsrs::FSRS::default())))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_schedule_timestamp(
        fsrs: *const Self,
        card: *const Card,
        now: i64,
    ) -> ScheduledCards {
        let fsrs_inner = unsafe { &*(*fsrs).0 };
        let card_inner = unsafe { &*(*card).0 };
        ScheduledCards(Box::into_raw(Box::new(fsrs_inner.schedule(
            card_inner.clone(),
            DateTime::from_timestamp(now, 0).expect("err"),
        ))))
    }
}
#[repr(C)]
#[derive(Clone)]
pub struct Card(*const fsrs::Card);

impl Card {
    #[no_mangle]
    pub extern "C" fn fsrs_Card_new() -> Self {
        Self(Box::into_raw(Box::new(fsrs::Card::new())))
    }
}

#[repr(C)]
pub struct ScheduledCards(*const fsrs::ScheduledCards);
impl ScheduledCards {
    #[no_mangle]
    pub extern "C" fn fsrs_ScheduledCards_select_card(&self, r: Rating) -> Card {
        let s = unsafe { &*(*self).0 };
        Card(Box::into_raw(Box::new(s.select_card(r.into()))))
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Option_ReviewLog {
    pub log: ReviewLog,
    pub none: bool,
}

impl Option_ReviewLog {
    #[no_mangle]
    pub extern "C" fn fsrs_Card_log(s: *const Card) -> Self {
        let sc = unsafe { &*(*s).0 };
        let log = sc.log.clone();
        if let Some(log) = log {
            Self {
                log: ReviewLog {
                    elapsed_days: log.elapsed_days,
                    scheduled_days: log.scheduled_days,
                    reviewed_date_s: log.reviewed_date.timestamp(),
                    rating: log.rating.into(),
                    state: log.state.into(),
                },
                none: false,
            }
        } else {
            Self {
                log: ReviewLog::default(),
                none: true,
            }
        }
    }
}

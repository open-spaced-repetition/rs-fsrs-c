use chrono::DateTime;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct fsrs_Parameters {
    pub request_retention: f32,
    pub maximum_interval: i32,
    pub w: [f32; 19],
}

impl From<fsrs_Parameters> for fsrs::Parameters {
    fn from(value: fsrs_Parameters) -> Self {
        Self {
            request_retention: value.request_retention,
            maximum_interval: value.maximum_interval,
            w: value.w,
        }
    }
}

#[repr(C)]
pub struct fsrs_Fsrs(*const fsrs::FSRS);

impl fsrs_Fsrs {
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_new(p: fsrs_Parameters) -> Self {
        Self(Box::into_raw(Box::new(fsrs::FSRS::new(p.into()))))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_Fsrs_default() -> Self {
        Self(Box::into_raw(Box::new(fsrs::FSRS::default())))
    }
    #[no_mangle]
    pub extern "C" fn fsrs_schedule_timestamp(
        fsrs: *const Self,
        card: *const fsrs_Card,
        now: i64,
    ) -> fsrs_ScheduledCards {
        let fsrs_inner = unsafe { &*(*fsrs).0 };
        let card_inner = unsafe { &*(*card).0 };
        fsrs_ScheduledCards(Box::into_raw(Box::new(fsrs_inner.schedule(
            card_inner.clone(),
            DateTime::from_timestamp(now, 0).expect("err"),
        ))))
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fsrs_Card(*const fsrs::Card);
impl fsrs_Card {
    #[no_mangle]
    pub extern "C" fn fsrs_Card_new() -> Self {
        Self(Box::into_raw(Box::new(fsrs::Card::new())))
    }
}

#[repr(C)]
pub struct fsrs_ScheduledCards(*const fsrs::ScheduledCards);
impl fsrs_ScheduledCards {
    #[no_mangle]
    pub extern "C" fn select_card(&self, r: fsrs_Rating) -> fsrs_Card {
        let s = unsafe { &*(*self).0 };
        fsrs_Card(Box::into_raw(Box::new(s.select_card(r.into()))))
    }
}

#[repr(C)]
pub enum fsrs_Rating {
    Again = 1,
    Hard = 2,
    Good = 3,
    Easy = 4,
}
impl From<fsrs_Rating> for fsrs::Rating {
    fn from(value: fsrs_Rating) -> Self {
        use fsrs::Rating as r;
        use fsrs_Rating::*;
        match value {
            Again => r::Again,
            Hard => r::Hard,
            Good => r::Good,
            Easy => r::Easy,
        }
    }
}
impl From<fsrs::Rating> for fsrs_Rating {
    fn from(value: fsrs::Rating) -> Self {
        use fsrs::Rating::*;
        use fsrs_Rating as r;
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
pub enum fsrs_State {
    #[default]
    New = 0,
    Learning = 1,
    Review = 2,
    Relearning = 3,
}

impl From<fsrs_State> for fsrs::State {
    fn from(value: fsrs_State) -> Self {
        use fsrs::State as r;
        use fsrs_State::*;
        match value {
            New => r::New,
            Learning => r::Learning,
            Review => r::Review,
            Relearning => r::Relearning,
        }
    }
}

impl From<fsrs::State> for fsrs_State {
    fn from(value: fsrs::State) -> Self {
        use fsrs::State::*;
        use fsrs_State as r;
        match value {
            New => r::New,
            Learning => r::Learning,
            Review => r::Review,
            Relearning => r::Relearning,
        }
    }
}
#[repr(C)]
pub struct fsrs_ReviewLog {
    pub rating: fsrs_Rating,
    pub elapsed_days: i64,
    pub scheduled_days: i64,
    pub state: fsrs_State,
    pub reviewed_date_s: i64,
}

impl fsrs_ReviewLog {
    #[no_mangle]
    pub extern "C" fn fsrs_get_ReviewLog(s: *const fsrs_Card) -> Self {
        let sc = unsafe { &*(*s).0 };
        let log = sc.log.clone().unwrap();
        Self {
            elapsed_days: log.elapsed_days,
            scheduled_days: log.scheduled_days,
            reviewed_date_s: log.reviewed_date.timestamp(),
            rating: log.rating.into(),
            state: log.state.into(),
        }
    }
}

use chrono::{DateTime, Utc};

#[repr(C)]
pub struct fsrs_fsrs {
    inner: *const fsrs::FSRS,
}
impl fsrs_fsrs {
    #[no_mangle]
    pub extern "C" fn fsrs_fsrs_new() -> Self {
        Self {
            inner: Box::into_raw(Box::new(fsrs::FSRS::default())),
        }
    }
    #[no_mangle]
    pub extern "C" fn fsrs_schedule(
        fsrs: *const Self,
        card: *const fsrs_card,
    ) -> fsrs_ScheduledCards {
        let fsrs_inner = unsafe { &*(*fsrs).inner };
        let card_inner = unsafe { &*(*card).inner };
        fsrs_ScheduledCards {
            inner: Box::into_raw(Box::new(
                fsrs_inner.schedule(card_inner.clone(), Utc::now()),
            )),
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct fsrs_card {
    inner: *const fsrs::Card,
}
impl fsrs_card {
    #[no_mangle]
    pub extern "C" fn fsrs_card_new() -> Self {
        Self {
            inner: Box::into_raw(Box::new(fsrs::Card::new())),
        }
    }
    #[no_mangle]
    pub extern "C" fn fsrs_schedule_debug(&self) -> i64 {
        let sc = unsafe { &*(*self).inner };
        let ret = sc.log.clone().unwrap();
        return ret.scheduled_days;
    }
}

#[repr(C)]
pub struct fsrs_ScheduledCards {
    inner: *const fsrs::ScheduledCards,
}
impl fsrs_ScheduledCards {
    #[no_mangle]
    pub extern "C" fn select_card(&self, r: fsrs_Rating) -> fsrs_card {
        let s = unsafe { &*(*self).inner };
        fsrs_card {
            inner: Box::into_raw(Box::new(s.select_card(r.into()))),
        }
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

use crate::facilities::{Facility, FacilityVisualState};

pub(crate) struct Cursor{
    multiplier: f64,
    amount: u32,
}

impl Cursor {
    const BASE_CPS: f64 = 0.1;
}



impl Facility for Cursor {
    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Displayed
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn base_cost(&self) -> u128 {
        15
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            amount: 0,
        }
    }
}
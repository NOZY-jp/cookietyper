use bnum::types::U512;

use crate::facilities::{Facility, FacilityKey, FacilityVisualState};

pub(crate) struct Cursor {
    multiplier: f64,
    amount: u32,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            amount: 0,
        }
    }
}

impl Facility for Cursor {
    fn key() -> FacilityKey {
        FacilityKey::Cursor
    }

    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Covered
    }

    fn base_cost(&self) -> U512 {
        15u32.into()
    }

    fn base_cps(&self) -> f64 {
        0.1
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

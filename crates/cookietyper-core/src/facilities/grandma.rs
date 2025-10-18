use bnum::types::U512;

use crate::facilities::{Facility, FacilityKey, FacilityVisualState};

pub(crate) struct Grandma {
    amount: u32,
    multiplier: f64,
}

impl Default for Grandma {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            amount: 0,
        }
    }
}

impl Facility for Grandma {
    fn key() -> FacilityKey {
        FacilityKey::Grandma
    }

    fn visual_state(&self) -> super::FacilityVisualState {
        FacilityVisualState::Covered
    }

    fn base_cost(&self) -> U512 {
        100u32.into()
    }

    fn base_cps(&self) -> f64 {
        1.0
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

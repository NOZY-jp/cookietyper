use bnum::{
    cast::CastFrom as _,
    types::{I512, U512},
};

use crate::facilities::{Facility, FacilityKey, FacilityVisualState};

pub(crate) struct Cursor {
    multiplier: f64,
    amount: u32,
}

impl Cursor {
    const BASE_CPS: f64 = 0.1;
}

impl Facility for Cursor {
    fn key() -> FacilityKey {
        FacilityKey::Cursor
    }

    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Displayed
    }

    fn amount(&self) -> u32 {
        self.amount
    }

    fn base_cost(&self) -> U512 {
        15u32.into()
    }

    fn cps(&self) -> bnum::types::I512 {
        let cps = Cursor::BASE_CPS * self.multiplier;
        I512::cast_from(cps)
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

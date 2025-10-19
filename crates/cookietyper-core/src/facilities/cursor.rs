use crate::facilities::{FacilityHandlers, FacilityStatus, FacilityVisualState};
use bnum::types::U512;
use cookietyper_macro::Facility;

#[derive(Facility)]
#[facility(key = "Cursor", base_cost = 15, base_cps = 0.1)]
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

impl FacilityStatus for Cursor {
    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Covered
    }
}

impl FacilityHandlers for Cursor {}

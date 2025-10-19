use crate::facilities::{FacilityHandlers, FacilityStatus, FacilityVisualState};
use bnum::types::U512;
use cookietyper_macro::Facility;

#[derive(Facility)]
#[facility(key = "Grandma", base_cost = 100, base_cps = 1)]
pub(crate) struct Grandma {
    multiplier: f64,
    amount: u32,
}

impl Default for Grandma {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            amount: 0,
        }
    }
}

impl FacilityStatus for Grandma {
    fn visual_state(&self) -> FacilityVisualState {
        FacilityVisualState::Covered
    }
}

impl FacilityHandlers for Grandma {}

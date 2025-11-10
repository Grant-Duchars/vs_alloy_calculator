#![allow(dead_code)]
//! # Vintage Story Alloy Calculator lib crate
//! An overly engineered library used for modeling values for all the alloys in the game [Vintage Story](https://www.vintagestory.at/).
//!
//! ## Examples
//! ```rust
//! use vs_alloy_calculator::prelude::*;
//!
//! let alloy = AlloyData::<TinBronze>::try_new([Copper(0.92), Tin(0.08)], 7).expect("should be valid");
//! let nuggets = alloy.nuggets();
//!
//! assert_eq!(Copper(128), nuggets[0]);
//! assert_eq!(Tin(12), nuggets[1]);
//! ```
//! ```rust
//! use vs_alloy_calculator::prelude::*;
//!
//! let input = { // Mock getting input from user
//!     // Show the user the valid ranges for the alloy
//!     let ranges = AlloyData::<BismuthBronze>::percentage_ranges();
//!     // Get the input back from the user
//!     (Box::from([Copper(0.60), Zinc(0.20), Bismuth(0.20)]), 13)
//! };
//!
//! let alloy = AlloyData::<BismuthBronze>::try_new(input.0, input.1).expect("should be valid");
//! assert_eq!(
//!     &[Copper(156), Zinc(52), Bismuth(52)],
//!     alloy.nuggets(),
//! );
//! ```

// Modules
mod alloy;
mod base_metal;
mod tests;
// Re-exports
pub use alloy::Alloy;
pub use alloy::AlloyData;
pub use alloy::alloy_types;
pub use alloy::alloy_types::AlloyType;
pub use base_metal::BaseMetal;
pub mod prelude {
    pub use crate::alloy::Alloy;
    pub use crate::alloy::AlloyData;
    // Gives AlloyType and all of the types of alloys as standalone types and as an enum
    pub use crate::alloy::alloy_types::*;
    pub use crate::base_metal::BaseMetal;
    pub use crate::base_metal::BaseMetal::*;
}
// Imports
use AlloyError::*;
use BaseMetal::*;

pub mod unit_constants {
    pub const NUGGET_UNIT_AMOUNT: i32 = 5;
    pub const INGOT_UNIT_AMOUNT: i32 = 100;
    pub const MAX_STACK_SIZE: i32 = 128;
    pub const CRUCIBLE_SLOTS: i32 = 4;
    pub const MAX_POSSIBLE_INGOTS: i32 =
        MAX_STACK_SIZE * NUGGET_UNIT_AMOUNT * CRUCIBLE_SLOTS / INGOT_UNIT_AMOUNT; // 25
    pub const MAX_UNITS_PER_SLOT: i32 = MAX_STACK_SIZE * NUGGET_UNIT_AMOUNT; // 640
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlloyError {
    InvalidPercentages,
    InvalidConstituentAmounts,
    InvalidBaseMetals,
    InvalidValues,
    TooManyIngots,
    TooFewIngots,
}

/// Struct for modeling valid percentage ranges for constituents of an [`Alloy`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConstituentRange<T: Copy + PartialOrd> {
    pub min: T,
    pub max: T,
}
impl<T: Copy + PartialOrd> ConstituentRange<T> {
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, item: &T) -> bool {
        self.min <= *item && *item <= self.max
    }
}
type Range = ConstituentRange<f32>;

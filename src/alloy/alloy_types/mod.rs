//! # Alloy Types
//! This module contains the [AlloyType] trait which is used to give shared functionality to the alloy types also located in this module's child modules.

use super::*;

pub trait AlloyType: private::AlloyType {
    /// Checks if the supplied percentages are valid (eg. total to 1.0 and are within the ranges) \
    /// Returns the validated percentages in a reordered boxed slice or an error if invalid
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let percentages = [Tin(0.08), Copper(0.92)];
    /// assert!(
    ///     TinBronze::check_valid_percentages(percentages)
    ///         .is_ok_and(|p| p == Box::from([Copper(0.92), Tin(0.08)]))
    /// );
    /// ```
    fn check_valid_percentages(
        percentages: impl AsRef<[BaseMetal<f32>]>,
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        if Self::check_percentages_total(&percentages) {
            Self::check_ranges_contains(&percentages)
        } else {
            Err(InvalidPercentages)
        }
    }

    /// Checks if the supplied percentages total to 1.0
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let percentages = [Copper(0.92), Tin(0.08)];
    /// let validation = TinBronze::check_percentages_total(percentages);
    ///
    /// assert!(validation);
    /// ```
    fn check_percentages_total(percentages: impl AsRef<[BaseMetal<f32>]>) -> bool {
        let sum = percentages.as_ref().iter().map(|p| **p).sum::<f32>();
        (sum - 1.0).abs() < 0.01
    }

    /// Checks if the supplied percentages are within the ranges for the alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let percentages = [Copper(0.92), Tin(0.08)];
    /// let validation = TinBronze::check_ranges_contains(percentages).is_ok();
    ///
    /// assert!(validation);
    /// ```
    fn check_ranges_contains(
        percentages: impl AsRef<[BaseMetal<f32>]>,
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let percentages = percentages.as_ref();
        match percentages.len() {
            l if l == 2 || l == 3 => {
                let perc_1 = percentages[0];
                let perc_2 = percentages[1];
                let percentages = if let Some(perc_3) = percentages.get(2) {
                    Self::check_own_ranges_contains(&[perc_1, perc_2, *perc_3])?
                } else {
                    Self::check_own_ranges_contains(&[perc_1, perc_2])?
                };
                Ok(percentages)
            }
            _ => Err(InvalidPercentages),
        }
    }

    /// Returns the name of the alloy as a string
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let name = TinBronze::name();
    ///
    /// assert_eq!("Tin Bronze", name);
    /// ```
    fn name() -> &'static str {
        Self::NAME
    }

    /// Returns constituent percentage ranges in order of largest to smallest
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    /// use vs_alloy_calculator::ConstituentRange;
    ///
    /// let ranges = TinBronze::percentage_ranges();
    ///
    /// assert_eq!(&[Copper(ConstituentRange::new(0.88, 0.92)), Tin(ConstituentRange::new(0.08, 0.12))], ranges);
    /// ```
    fn percentage_ranges() -> &'static [BaseMetal<Range>] {
        Self::RANGES
    }

    /// Returns the array of constituent nugget amounts
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloy::<TinBronze>::default();
    /// let nuggets = alloy.constituents().nuggets();
    ///
    /// assert_eq!(&[Copper(18), Tin(2)], nuggets);
    /// ```
    fn nuggets(&self) -> &[BaseMetal<i32>];
}

mod private {
    use super::*;
    pub trait AlloyType: Sized {
        const NAME: &str;
        const RANGES: &[BaseMetal<Range>];

        fn check_base_metal(value: &f32, index: usize, seen: bool) -> Result<(), AlloyError> {
            if seen {
                Err(InvalidBaseMetals)
            } else if !Self::RANGES[index].contains(value) {
                Err(InvalidPercentages)
            } else {
                Ok(())
            }
        }

        fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError>;
        fn check_own_ranges_contains(
            percentages: &[BaseMetal<f32>],
        ) -> Result<Box<[BaseMetal<f32>]>, AlloyError>;
    }
}

/// Enum of the available alloys. If you are trying to generate values for a given alloy, use the [`Alloy`] struct
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Alloys {
    TinBronze,
    BismuthBronze,
    BlackBronze,
    Brass,
    Molybdochalkos,
    LeadSolder,
    SilverSolder,
    Electrum,
    Cupronickel,
}

// Modules
mod bismuth_bronze;
mod black_bronze;
mod brass;
mod cupronickel;
mod electrum;
mod lead_solder;
mod molybdochalkos;
mod silver_solder;
mod tin_bronze;
// Re-exports
pub use bismuth_bronze::BismuthBronze;
pub use black_bronze::BlackBronze;
pub use brass::Brass;
pub use cupronickel::Cupronickel;
pub use electrum::Electrum;
pub use lead_solder::LeadSolder;
pub use molybdochalkos::Molybdochalkos;
pub use silver_solder::SilverSolder;
pub use tin_bronze::TinBronze;

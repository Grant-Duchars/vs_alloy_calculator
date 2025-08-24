use super::*;
pub mod alloy_types;

/// Struct for modeling all of the alloys in Vintage Story
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Alloy<T: AlloyType> {
    /// Also stores number of nuggets of each constituent
    alloy_type: T,
    percentages: Box<[BaseMetal<f32>]>,
    num_ingots: i32,
    max_ingots: i32,
}

impl<T: AlloyType> Alloy<T> {
    /// Tries to create a new instance of an alloy. Checks if the input values are valid and tries to calculate valid values for the given alloy.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let percentages = [Copper(0.92), Tin(0.08)];
    /// let num_ingots = 7;
    ///
    /// let alloy = Alloy::<TinBronze>::try_new(percentages, num_ingots).expect("should be valid");
    ///
    /// assert_eq!(&[Copper(128), Tin(12)], alloy.constituents().nuggets());
    /// ```
    pub fn try_new(
        percentages: impl AsRef<[BaseMetal<f32>]>,
        num_ingots: i32,
    ) -> Result<Self, AlloyError> {
        if num_ingots > unit_constants::MAX_POSSIBLE_INGOTS {
            Err(TooManyIngots)
        } else if num_ingots == 0 {
            Err(TooFewIngots)
        } else {
            match T::check_valid_percentages(&percentages) {
                Ok(percentages) => {
                    let (alloy_type, max_ingots) =
                        Self::get_updated_values(&percentages, num_ingots)?;
                    Ok(Self {
                        alloy_type,
                        percentages,
                        num_ingots,
                        max_ingots,
                    })
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Gets the underlying alloy type which holds the current constituent nugget amounts. \
    /// To get at the actual amounts, `.nuggets()` must be called in addition.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloy::<TinBronze>::default();
    /// let alloy_type = alloy.constituents();
    /// let nuggets = alloy_type.nuggets();
    ///
    /// assert_eq!(&[Copper(18), Tin(2)], nuggets);
    /// ```
    pub fn constituents(&self) -> &T {
        &self.alloy_type
    }

    /// Gets the number of ingots that are able to be created with the current constituent amounts
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloy::<TinBronze>::default();
    /// let num_ingots = alloy.num_ingots();
    ///
    /// assert_eq!(1, num_ingots);
    /// ```
    pub fn num_ingots(&self) -> i32 {
        self.num_ingots
    }

    /// Gets the maximum number of ingots possible with the current percentages of the alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloy::<TinBronze>::default();
    /// let max_ingots = alloy.max_ingots();
    ///
    /// assert_eq!(20, max_ingots);
    /// ```
    pub fn max_ingots(&self) -> i32 {
        self.max_ingots
    }

    /// Gets the percentages of base metals of the alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloy::<TinBronze>::default();
    /// let percentages = alloy.percentages();
    ///
    /// assert_eq!(&[Copper(0.92), Tin(0.08)], percentages);
    /// ```
    pub fn percentages(&self) -> &[BaseMetal<f32>] {
        &self.percentages
    }

    /// Gets the ranges of percentages of base metals for the given alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    /// use vs_alloy_calculator::ConstituentRange;
    ///
    /// let ranges = Alloy::<TinBronze>::percentage_ranges();
    ///
    /// assert_eq!(&[Copper(ConstituentRange::new(0.88, 0.92)), Tin(ConstituentRange::new(0.08, 0.12))], ranges);
    /// ```
    pub fn percentage_ranges() -> &'static [BaseMetal<Range>] {
        T::percentage_ranges()
    }

    /// Tries to update the number of ingots for the alloy. In addition, updates other values if successful.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let mut alloy = Alloy::<TinBronze>::default();
    /// assert_eq!(1, alloy.num_ingots());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.constituents().nuggets());
    ///
    /// // Updating the number of ingots also calculates and updates other values too
    /// alloy.set_num_ingots(5).expect("should be valid");
    /// assert_eq!(5, alloy.num_ingots());
    /// assert_eq!(&[Copper(92), Tin(8)], alloy.constituents().nuggets());
    ///
    /// // Returns an error and does not update any values if the number of ingots is too high or too low
    /// alloy.set_num_ingots(100).expect_err("should be too many ingots");
    /// alloy.set_num_ingots(0).expect_err("should be too few ingots");
    /// assert_eq!(5, alloy.num_ingots()); // Values were not updated
    /// assert_eq!(&[Copper(92), Tin(8)], alloy.constituents().nuggets());
    /// ```
    pub fn set_num_ingots(&mut self, num_ingots: i32) -> Result<(), AlloyError> {
        if num_ingots == 0 {
            Err(TooFewIngots)
        } else if self.max_ingots < num_ingots {
            Err(TooManyIngots)
        } else {
            self.update_values(None, Some(num_ingots))?;
            Ok(())
        }
    }

    /// Tries to update the percentages for the alloy. In addition, updates other values if successful.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let mut alloy = Alloy::<TinBronze>::default();
    /// assert_eq!(&[Copper(0.92), Tin(0.08)], alloy.percentages());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.constituents().nuggets());
    /// assert_eq!(20, alloy.max_ingots());
    ///
    /// // Updating the percentages also calculates and updates other values too
    /// alloy.set_percentages([Copper(0.88), Tin(0.12)]).expect("should be valid");
    /// assert_eq!(&[Copper(0.88), Tin(0.12)], alloy.percentages());
    /// assert_eq!(&[Copper(17), Tin(3)], alloy.constituents().nuggets());
    /// assert_eq!(21, alloy.max_ingots());
    ///
    /// // Returns an error and does not update any values if the percentages are invalid for the alloy
    /// alloy.set_percentages([Copper(0.12), Tin(0.88)]).expect_err("should be invalid ranges");
    /// alloy.set_percentages([Lead(0.92), Copper(0.08)]).expect_err("should be invalid base metals");
    /// assert_eq!(&[Copper(0.88), Tin(0.12)], alloy.percentages()); // Values were not updated
    /// assert_eq!(&[Copper(17), Tin(3)], alloy.constituents().nuggets());
    /// ```
    pub fn set_percentages(
        &mut self,
        percentages: impl AsRef<[BaseMetal<f32>]>,
    ) -> Result<(), AlloyError> {
        match T::check_valid_percentages(&percentages) {
            Ok(p) => {
                self.update_values(Some(p), None)?;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Checks whether the supplied percentages are valid for the given alloy \
    /// Returns the validated percentages in a reordered boxed slice or an error if invalid
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let percentages = [Tin(0.08), Copper(0.92)];
    /// assert!(
    ///     Alloy::<TinBronze>::check_valid_percentages(percentages)
    ///         .is_ok_and(|p| p == Box::from([Copper(0.92), Tin(0.08)]))
    /// );
    /// ```
    pub fn check_valid_percentages(
        percentages: impl AsRef<[BaseMetal<f32>]>,
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        T::check_valid_percentages(percentages)
    }

    /// Tries to update the alloy's values. Should pass in either percentage, num_ingots, or both but never neither.
    fn update_values(
        &mut self,
        percentages: Option<Box<[BaseMetal<f32>]>>,
        num_ingots: Option<i32>,
    ) -> Result<(), AlloyError> {
        let (alloy_type, max_ingots) = match (percentages, num_ingots) {
            (Some(percentages), Some(num_ingots)) => {
                let update = Self::get_updated_values(&percentages, num_ingots)?;
                self.percentages = percentages;
                self.num_ingots = num_ingots;
                update
            }
            (Some(percentages), None) => {
                let update = Self::get_updated_values(&percentages, self.num_ingots)?;
                self.percentages = percentages;
                update
            }
            (None, Some(num_ingots)) => {
                let update = Self::get_updated_values(&self.percentages, num_ingots)?;
                self.num_ingots = num_ingots;
                update
            }
            (None, None) => return Err(InvalidValues),
        };
        self.alloy_type = alloy_type;
        self.max_ingots = max_ingots;
        Ok(())
    }

    /// Gets updated values using the supplied parameters
    fn get_updated_values(
        percentages: &[BaseMetal<f32>],
        num_ingots: i32,
    ) -> Result<(T, i32), AlloyError> {
        use AlloyError::*;
        use unit_constants::*;

        // Constituent Amounts
        let needed_units_c = num_ingots as f32 * INGOT_UNIT_AMOUNT as f32;
        let mut remaining_units_c = needed_units_c;
        let mut constituent_amounts = Vec::new();
        // Max Ingots
        let needed_units_mi = MAX_POSSIBLE_INGOTS as f32 * INGOT_UNIT_AMOUNT as f32;
        let mut remaining_units_mi = needed_units_mi;

        let len = percentages.len();
        let slots_used = percentages.iter().enumerate().fold(0.0, |acc, (i, p)| {
            let p = **p;
            // Constituent Amounts
            if i < len - 1 {
                let units = needed_units_c * p;
                remaining_units_c -= units;
                constituent_amounts.push(units as i32 / NUGGET_UNIT_AMOUNT);
            } else {
                constituent_amounts
                    .push((remaining_units_c / NUGGET_UNIT_AMOUNT as f32).ceil() as i32);
            }
            // Max Ingots
            let units = needed_units_mi * p;
            remaining_units_mi -= units;
            acc + (units / MAX_UNITS_PER_SLOT as f32).ceil()
        });

        // Recursively run calculate_max_ingots until max_ingots is calculated
        let max_ingots = if remaining_units_mi != 0.0 || slots_used > 4.0 {
            Self::calculate_max_ingots(percentages, MAX_POSSIBLE_INGOTS - 1)
        } else {
            MAX_POSSIBLE_INGOTS
        };

        if num_ingots > max_ingots {
            return Err(TooManyIngots);
        }

        Ok((T::try_from_vec(constituent_amounts)?, max_ingots))
    }

    /// Calculates the maximum number of ingots possible with the supplied constituent percentages starting at high value and working downwards
    fn calculate_max_ingots(percentages: &[BaseMetal<f32>], cur_ingot_num: i32) -> i32 {
        let needed_units = cur_ingot_num as f32 * unit_constants::INGOT_UNIT_AMOUNT as f32;
        let mut remaining_units = needed_units;

        let slots_used = percentages.as_ref().iter().fold(0.0, |acc, p| {
            let units = needed_units * **p;
            remaining_units -= units;
            acc + (units / unit_constants::MAX_UNITS_PER_SLOT as f32).ceil()
        });

        if remaining_units != 0.0 || slots_used > 4.0 {
            Self::calculate_max_ingots(percentages, cur_ingot_num - 1)
        } else {
            cur_ingot_num
        }
    }
}

use super::*;
use alloy_types::*;
pub mod alloy_types;

pub mod alloy_names {
    pub const TIN_BRONZE: &str = "Tin Bronze";
    pub const BISMUTH_BRONZE: &str = "Bismuth Bronze";
    pub const BLACK_BRONZE: &str = "Black Bronze";
    pub const BRASS: &str = "Brass";
    pub const MOLYBDOCHALKOS: &str = "Molybdochalkos";
    pub const LEAD_SOLDER: &str = "Lead Solder";
    pub const SILVER_SOLDER: &str = "Silver Solder";
    pub const ELECTRUM: &str = "Electrum";
    pub const CUPRONICKEL: &str = "Cupronickel";
}

/// Unified alloy enum
pub enum Alloy {
    TinBronze(AlloyData<TinBronze>),
    BismuthBronze(AlloyData<BismuthBronze>),
    BlackBronze(AlloyData<BlackBronze>),
    Brass(AlloyData<Brass>),
    Molybdochalkos(AlloyData<Molybdochalkos>),
    LeadSolder(AlloyData<LeadSolder>),
    SilverSolder(AlloyData<SilverSolder>),
    Electrum(AlloyData<Electrum>),
    Cupronickel(AlloyData<Cupronickel>),
}

/// Struct for modeling all of the alloys in Vintage Story
#[derive(PartialEq, PartialOrd, Debug)]
pub struct AlloyData<T: AlloyType> {
    /// Also stores number of nuggets of each constituent
    alloy_type: T,
    percentages: Box<[BaseMetal<f32>]>,
    num_ingots: i32,
    max_ingots: i32,
}

impl<T: AlloyType> AlloyData<T> {
    /// Tries to create a new instance of an alloy. Checks if the input values are valid and tries to calculate valid values for the given alloy.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let percentages = [Copper(0.92), Tin(0.08)];
    /// let num_ingots = 7;
    ///
    /// let alloy = AlloyData::<TinBronze>::try_new(percentages, num_ingots).expect("should be valid");
    ///
    /// assert_eq!(&[Copper(128), Tin(12)], alloy.nuggets());
    /// ```
    pub fn try_new(
        percentages: impl AsRef<[BaseMetal<f32>]>,
        num_ingots: i32,
    ) -> Result<Self, AlloyError> {
        if num_ingots > unit_constants::MAX_POSSIBLE_INGOTS {
            Err(TooManyIngots)
        } else if num_ingots <= 0 {
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

    /// Gets the number of nuggets of each constituent needed to create the current number of ingots with the current constituent ratios
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = AlloyData::<TinBronze>::default();
    /// let nuggets = alloy.nuggets();
    ///
    /// assert_eq!(&[Copper(18), Tin(2)], nuggets);
    /// ```
    pub fn nuggets(&self) -> &[BaseMetal<i32>] {
        self.alloy_type.nuggets()
    }

    /// Gets the number of ingots that are able to be created with the current constituent amounts
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = AlloyData::<TinBronze>::default();
    /// let num_ingots = alloy.num_ingots();
    ///
    /// assert_eq!(1, num_ingots);
    /// ```
    pub fn num_ingots(&self) -> i32 {
        self.num_ingots
    }

    /// Gets the maximum number of ingots possible with the current constituent percentages
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = AlloyData::<TinBronze>::default();
    /// let max_ingots = alloy.max_ingots();
    ///
    /// assert_eq!(20, max_ingots);
    /// ```
    pub fn max_ingots(&self) -> i32 {
        self.max_ingots
    }

    /// Gets the percentages of the constituents of the alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = AlloyData::<TinBronze>::default();
    /// let percentages = alloy.percentages();
    ///
    /// assert_eq!(&[Copper(0.92), Tin(0.08)], percentages);
    /// ```
    pub fn percentages(&self) -> &[BaseMetal<f32>] {
        &self.percentages
    }

    /// Gets the ranges of percentages of the constituents for the given alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    /// use vs_alloy_calculator::ConstituentRange;
    ///
    /// let ranges = AlloyData::<TinBronze>::percentage_ranges();
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
    /// let mut alloy = AlloyData::<TinBronze>::default();
    /// assert_eq!(1, alloy.num_ingots());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    ///
    /// // Updating the number of ingots also calculates and updates other values too
    /// alloy.set_num_ingots(5).expect("should be valid");
    /// assert_eq!(5, alloy.num_ingots());
    /// assert_eq!(&[Copper(92), Tin(8)], alloy.nuggets());
    ///
    /// // Returns an error and does not update any values if the number of ingots is too high or too low
    /// alloy.set_num_ingots(100).expect_err("should be too many ingots");
    /// alloy.set_num_ingots(0).expect_err("should be too few ingots");
    /// assert_eq!(5, alloy.num_ingots()); // Values were not updated
    /// assert_eq!(&[Copper(92), Tin(8)], alloy.nuggets());
    /// ```
    pub fn set_num_ingots(&mut self, num_ingots: i32) -> Result<(), AlloyError> {
        if num_ingots <= 0 {
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
    /// let mut alloy = AlloyData::<TinBronze>::default();
    /// assert_eq!(&[Copper(0.92), Tin(0.08)], alloy.percentages());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    /// assert_eq!(20, alloy.max_ingots());
    ///
    /// // Updating the percentages also calculates and updates other values too
    /// alloy.set_percentages([Copper(0.88), Tin(0.12)]).expect("should be valid");
    /// assert_eq!(&[Copper(0.88), Tin(0.12)], alloy.percentages());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    /// assert_eq!(21, alloy.max_ingots());
    ///
    /// // Returns an error and does not update any values if the percentages are invalid for the alloy
    /// alloy.set_percentages([Copper(0.12), Tin(0.88)]).expect_err("should be invalid ranges");
    /// alloy.set_percentages([Lead(0.92), Copper(0.08)]).expect_err("should be invalid base metals");
    /// assert_eq!(&[Copper(0.88), Tin(0.12)], alloy.percentages()); // Values were not updated
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
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
    ///     AlloyData::<TinBronze>::check_valid_percentages(percentages)
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
        let needed_units_c = (num_ingots * INGOT_UNIT_AMOUNT) as f32;
        let mut remaining_units_c = needed_units_c;
        let mut constituent_amounts = Vec::new();
        // Max Ingots
        let needed_units_mi = (MAX_POSSIBLE_INGOTS * INGOT_UNIT_AMOUNT) as f32;
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

        Self::check_constituent_amounts(&mut constituent_amounts, num_ingots);

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

    fn check_constituent_amounts(amounts: &mut Vec<i32>, num_ingots: i32) {
        use unit_constants::*;

        let two_constituents = amounts.len() == 2;
        let invalid_sum = amounts.iter().sum::<i32>() != NUM_NUGGETS_PER_INGOT * num_ingots;

        let (a, amounts) = amounts.split_at_mut(1);
        let a = &mut a[0];
        let (b, c) = amounts.split_at_mut(1);
        let b = &mut b[0];

        let ranges = Self::percentage_ranges();
        let max_b = (ranges[1].max * (NUM_NUGGETS_PER_INGOT * num_ingots) as f32).floor() as i32;

        if two_constituents {
            if invalid_sum {
                if *b < max_b {
                    *b += 1;
                } else {
                    *a += 1;
                }
            } else if *b > max_b {
                *b -= 1;
                *a += 1;
            }
        } else {
            let c = &mut c[0];
            let max_c =
                (ranges[2].max * (NUM_NUGGETS_PER_INGOT * num_ingots) as f32).floor() as i32;

            if invalid_sum {
                if *c < max_c {
                    *c += 1;
                } else if *b < max_b {
                    *b += 1;
                } else {
                    *a += 1;
                }
            } else if *c > max_c {
                *c -= 1;
                if *b + 1 > max_b {
                    *a += 1;
                } else {
                    *b += 1;
                }
            }
        }
    }
}

impl Alloy {
    /// Returns the name of the alloy as a string
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloys::TinBronze.get_default();
    /// let name = alloy.name();
    ///
    /// assert_eq!("Tin Bronze", name);
    /// ```
    pub fn name(&self) -> &str {
        use alloy_names::*;
        match self {
            Alloy::TinBronze(_) => TIN_BRONZE,
            Alloy::BismuthBronze(_) => BISMUTH_BRONZE,
            Alloy::BlackBronze(_) => BLACK_BRONZE,
            Alloy::Brass(_) => BRASS,
            Alloy::Molybdochalkos(_) => MOLYBDOCHALKOS,
            Alloy::LeadSolder(_) => LEAD_SOLDER,
            Alloy::SilverSolder(_) => SILVER_SOLDER,
            Alloy::Electrum(_) => ELECTRUM,
            Alloy::Cupronickel(_) => CUPRONICKEL,
        }
    }

    /// Gets the number of nuggets of each constituent needed to create the current number of ingots with the current constituent ratios
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloys::TinBronze.get_default();
    /// let nuggets = alloy.nuggets();
    ///
    /// assert_eq!(&[Copper(18), Tin(2)], nuggets);
    /// ```
    pub fn nuggets(&self) -> &[BaseMetal<i32>] {
        match self {
            Alloy::TinBronze(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::BismuthBronze(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::BlackBronze(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::Brass(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::Molybdochalkos(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::LeadSolder(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::SilverSolder(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::Electrum(alloy_data) => alloy_data.alloy_type.nuggets(),
            Alloy::Cupronickel(alloy_data) => alloy_data.alloy_type.nuggets(),
        }
    }

    /// Gets the number of ingots that are able to be created with the current constituent amounts
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloys::TinBronze.get_default();
    /// let num_ingots = alloy.num_ingots();
    ///
    /// assert_eq!(1, num_ingots);
    /// ```
    pub fn num_ingots(&self) -> i32 {
        match self {
            Alloy::TinBronze(alloy_data) => alloy_data.num_ingots,
            Alloy::BismuthBronze(alloy_data) => alloy_data.num_ingots,
            Alloy::BlackBronze(alloy_data) => alloy_data.num_ingots,
            Alloy::Brass(alloy_data) => alloy_data.num_ingots,
            Alloy::Molybdochalkos(alloy_data) => alloy_data.num_ingots,
            Alloy::LeadSolder(alloy_data) => alloy_data.num_ingots,
            Alloy::SilverSolder(alloy_data) => alloy_data.num_ingots,
            Alloy::Electrum(alloy_data) => alloy_data.num_ingots,
            Alloy::Cupronickel(alloy_data) => alloy_data.num_ingots,
        }
    }

    /// Gets the maximum number of ingots possible with the current constituent percentages
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloys::TinBronze.get_default();
    /// let max_ingots = alloy.max_ingots();
    ///
    /// assert_eq!(20, max_ingots);
    /// ```
    pub fn max_ingots(&self) -> i32 {
        match self {
            Alloy::TinBronze(alloy_data) => alloy_data.max_ingots,
            Alloy::BismuthBronze(alloy_data) => alloy_data.max_ingots,
            Alloy::BlackBronze(alloy_data) => alloy_data.max_ingots,
            Alloy::Brass(alloy_data) => alloy_data.max_ingots,
            Alloy::Molybdochalkos(alloy_data) => alloy_data.max_ingots,
            Alloy::LeadSolder(alloy_data) => alloy_data.max_ingots,
            Alloy::SilverSolder(alloy_data) => alloy_data.max_ingots,
            Alloy::Electrum(alloy_data) => alloy_data.max_ingots,
            Alloy::Cupronickel(alloy_data) => alloy_data.max_ingots,
        }
    }

    /// Gets the percentages of the constituents of the alloy
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let alloy = Alloys::TinBronze.get_default();
    /// let percentages = alloy.percentages();
    ///
    /// assert_eq!(&[Copper(0.92), Tin(0.08)], percentages);
    /// ```
    pub fn percentages(&self) -> &[BaseMetal<f32>] {
        match self {
            Alloy::TinBronze(alloy_data) => &alloy_data.percentages,
            Alloy::BismuthBronze(alloy_data) => &alloy_data.percentages,
            Alloy::BlackBronze(alloy_data) => &alloy_data.percentages,
            Alloy::Brass(alloy_data) => &alloy_data.percentages,
            Alloy::Molybdochalkos(alloy_data) => &alloy_data.percentages,
            Alloy::LeadSolder(alloy_data) => &alloy_data.percentages,
            Alloy::SilverSolder(alloy_data) => &alloy_data.percentages,
            Alloy::Electrum(alloy_data) => &alloy_data.percentages,
            Alloy::Cupronickel(alloy_data) => &alloy_data.percentages,
        }
    }

    /// Tries to update the number of ingots for the alloy. In addition, updates other values if successful.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let mut alloy = Alloys::TinBronze.get_default();
    /// assert_eq!(1, alloy.num_ingots());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    ///
    /// // Updating the number of ingots also calculates and updates other values too
    /// alloy.set_num_ingots(5).expect("should be valid");
    /// assert_eq!(5, alloy.num_ingots());
    /// assert_eq!(&[Copper(92), Tin(8)], alloy.nuggets());
    ///
    /// // Returns an error and does not update any values if the number of ingots is too high or too low
    /// alloy.set_num_ingots(100).expect_err("should be too many ingots");
    /// alloy.set_num_ingots(0).expect_err("should be too few ingots");
    /// assert_eq!(5, alloy.num_ingots()); // Values were not updated
    /// assert_eq!(&[Copper(92), Tin(8)], alloy.nuggets());
    /// ```
    pub fn set_num_ingots(&mut self, num_ingots: i32) -> Result<(), AlloyError> {
        match self {
            Alloy::TinBronze(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::BismuthBronze(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::BlackBronze(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::Brass(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::Molybdochalkos(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::LeadSolder(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::SilverSolder(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::Electrum(alloy_data) => alloy_data.set_num_ingots(num_ingots),
            Alloy::Cupronickel(alloy_data) => alloy_data.set_num_ingots(num_ingots),
        }
    }

    /// Tries to update the percentages for the alloy. In addition, updates other values if successful.
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let mut alloy = Alloys::TinBronze.get_default();
    /// assert_eq!(&[Copper(0.92), Tin(0.08)], alloy.percentages());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    /// assert_eq!(20, alloy.max_ingots());
    ///
    /// // Updating the percentages also calculates and updates other values too
    /// alloy.set_percentages([Copper(0.88), Tin(0.12)]).expect("should be valid");
    /// assert_eq!(&[Copper(0.88), Tin(0.12)], alloy.percentages());
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    /// assert_eq!(21, alloy.max_ingots());
    ///
    /// // Returns an error and does not update any values if the percentages are invalid for the alloy
    /// alloy.set_percentages([Copper(0.12), Tin(0.88)]).expect_err("should be invalid ranges");
    /// alloy.set_percentages([Lead(0.92), Copper(0.08)]).expect_err("should be invalid base metals");
    /// assert_eq!(&[Copper(0.88), Tin(0.12)], alloy.percentages()); // Values were not updated
    /// assert_eq!(&[Copper(18), Tin(2)], alloy.nuggets());
    /// assert_eq!(21, alloy.max_ingots());
    /// ```
    pub fn set_percentages(
        &mut self,
        percentages: impl AsRef<[BaseMetal<f32>]>,
    ) -> Result<(), AlloyError> {
        match self {
            Alloy::TinBronze(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::BismuthBronze(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::BlackBronze(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::Brass(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::Molybdochalkos(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::LeadSolder(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::SilverSolder(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::Electrum(alloy_data) => alloy_data.set_percentages(percentages),
            Alloy::Cupronickel(alloy_data) => alloy_data.set_percentages(percentages),
        }
    }
}

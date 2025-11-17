const NICKEL: &str = "Nickel";
const COPPER: &str = "Copper";
const ZINC: &str = "Zinc";
const SILVER: &str = "Silver";
const TIN: &str = "Tin";
const GOLD: &str = "Gold";
const LEAD: &str = "Lead";
const BISMUTH: &str = "Bismuth";

/// Enum for modeling different values of base metals used to create an [`Alloy`](crate::Alloy)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum BaseMetal<T: Copy> {
    Nickel(T),
    Copper(T),
    Zinc(T),
    Silver(T),
    Tin(T),
    Gold(T),
    Lead(T),
    Bismuth(T),
}
use BaseMetal::*;

impl<T: Copy> BaseMetal<T> {
    /// Returns the name of the base metal variant as a string
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let copper = Copper(5);
    /// let name = copper.name();
    ///
    /// assert_eq!("Copper", name);
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Nickel(_) => NICKEL,
            Copper(_) => COPPER,
            Zinc(_) => ZINC,
            Silver(_) => SILVER,
            Tin(_) => TIN,
            Gold(_) => GOLD,
            Lead(_) => LEAD,
            Bismuth(_) => BISMUTH,
        }
    }

    /// Updates the base metal with the new value in place
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let mut copper = Copper(5);
    /// assert_eq!(5, *copper);
    ///
    /// copper.update(10);
    /// assert_eq!(10, *copper);
    /// ```
    pub fn update(&mut self, value: T) {
        match self {
            Nickel(_) => *self = Nickel(value),
            Copper(_) => *self = Copper(value),
            Zinc(_) => *self = Zinc(value),
            Silver(_) => *self = Silver(value),
            Tin(_) => *self = Tin(value),
            Gold(_) => *self = Gold(value),
            Lead(_) => *self = Lead(value),
            Bismuth(_) => *self = Bismuth(value),
        }
    }

    /// Creates a new instance of the same base metal but with a new inner value and/or type
    /// ### Example
    /// ```rust
    /// use vs_alloy_calculator::prelude::*;
    ///
    /// let copper: BaseMetal<i32> = Copper(5);
    /// assert_eq!(5, *copper);
    ///
    /// let copper: BaseMetal<f64> = copper.update_inner_value(10.0);
    /// assert_eq!(10.0, *copper);
    /// ```
    pub fn update_inner_value<U: Copy>(&self, value: U) -> BaseMetal<U> {
        match self {
            Nickel(_) => Nickel(value),
            Copper(_) => Copper(value),
            Zinc(_) => Zinc(value),
            Silver(_) => Silver(value),
            Tin(_) => Tin(value),
            Gold(_) => Gold(value),
            Lead(_) => Lead(value),
            Bismuth(_) => Bismuth(value),
        }
    }
}

impl<T: Copy> std::ops::Deref for BaseMetal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Nickel(t) => t,
            Copper(t) => t,
            Zinc(t) => t,
            Silver(t) => t,
            Tin(t) => t,
            Gold(t) => t,
            Lead(t) => t,
            Bismuth(t) => t,
        }
    }
}

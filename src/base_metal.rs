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

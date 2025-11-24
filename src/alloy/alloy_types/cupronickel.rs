use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cupronickel([BaseMetal<i32>; 2]);

impl Default for Cupronickel {
    fn default() -> Self {
        Self([Copper(15), Nickel(5)])
    }
}

impl Default for AlloyData<Cupronickel> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Copper(0.75), Nickel(0.25)].into(),
            num_ingots: 1,
            max_ingots: 25,
        }
    }
}

impl AlloyType for Cupronickel {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}
impl private::AlloyType for Cupronickel {
    const NAME: &str = alloy_names::CUPRONICKEL;
    const RANGES: &[BaseMetal<Range>] = &[
        Copper(Range::new(0.65, 0.75)),
        Nickel(Range::new(0.25, 0.35)),
    ];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Copper(value.next().ok_or(InvalidConstituentAmounts)?),
            Nickel(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_copper, mut seen_nickel) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Copper(c) => {
                    Self::check_base_metal(c, 0, seen_copper)?;
                    seen_copper = true;
                    reorder[0] = Copper(*c);
                }
                Nickel(n) => {
                    Self::check_base_metal(n, 1, seen_nickel)?;
                    seen_nickel = true;
                    reorder[1] = Nickel(*n);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

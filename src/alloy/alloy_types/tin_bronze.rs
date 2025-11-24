use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TinBronze([BaseMetal<i32>; 2]);

impl Default for TinBronze {
    fn default() -> Self {
        Self([Copper(18), Tin(2)])
    }
}

impl Default for AlloyData<TinBronze> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Copper(0.92), Tin(0.08)].into(),
            num_ingots: 1,
            max_ingots: 20,
        }
    }
}

impl AlloyType for TinBronze {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for TinBronze {
    const NAME: &str = alloy_names::TIN_BRONZE;
    const RANGES: &[BaseMetal<Range>] =
        &[Copper(Range::new(0.88, 0.92)), Tin(Range::new(0.08, 0.12))];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Copper(value.next().ok_or(InvalidConstituentAmounts)?),
            Tin(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_copper, mut seen_tin) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Copper(c) => {
                    Self::check_base_metal(c, 0, seen_copper)?;
                    seen_copper = true;
                    reorder[0] = Copper(*c);
                }
                Tin(t) => {
                    Self::check_base_metal(t, 1, seen_tin)?;
                    seen_tin = true;
                    reorder[1] = Tin(*t);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

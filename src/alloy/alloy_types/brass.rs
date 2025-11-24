use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Brass([BaseMetal<i32>; 2]);

impl Default for Brass {
    fn default() -> Self {
        Self([Copper(14), Zinc(6)])
    }
}

impl Default for AlloyData<Brass> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Copper(0.70), Zinc(0.30)].into(),
            num_ingots: 1,
            max_ingots: 21,
        }
    }
}

impl AlloyType for Brass {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for Brass {
    const NAME: &str = alloy_names::BRASS;
    const RANGES: &[BaseMetal<Range>] =
        &[Copper(Range::new(0.60, 0.70)), Zinc(Range::new(0.30, 0.40))];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Copper(value.next().ok_or(InvalidConstituentAmounts)?),
            Zinc(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_copper, mut seen_zinc) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Copper(c) => {
                    Self::check_base_metal(c, 0, seen_copper)?;
                    seen_copper = true;
                    reorder[0] = Copper(*c);
                }
                Zinc(z) => {
                    Self::check_base_metal(z, 1, seen_zinc)?;
                    seen_zinc = true;
                    reorder[1] = Zinc(*z);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

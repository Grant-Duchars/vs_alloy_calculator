use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Electrum([BaseMetal<i32>; 2]);

impl Default for Electrum {
    fn default() -> Self {
        Self([Gold(8), Silver(12)])
    }
}

impl Default for AlloyData<Electrum> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Gold(0.40), Silver(0.60)].into(),
            num_ingots: 1,
            max_ingots: 21,
        }
    }
}

impl AlloyType for Electrum {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for Electrum {
    const NAME: &str = alloy_names::ELECTRUM;
    const RANGES: &[BaseMetal<Range>] =
        &[Gold(Range::new(0.40, 0.60)), Silver(Range::new(0.40, 0.60))];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Gold(value.next().ok_or(InvalidConstituentAmounts)?),
            Silver(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_gold, mut seen_silver) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Gold(g) => {
                    Self::check_base_metal(g, 0, seen_gold)?;
                    seen_gold = true;
                    reorder[0] = Gold(*g);
                }
                Silver(s) => {
                    Self::check_base_metal(s, 1, seen_silver)?;
                    seen_silver = true;
                    reorder[1] = Silver(*s);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

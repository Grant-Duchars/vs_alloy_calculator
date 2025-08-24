use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SilverSolder([BaseMetal<i32>; 2]);

impl Default for SilverSolder {
    fn default() -> Self {
        Self([Tin(10), Silver(10)])
    }
}

impl Default for Alloy<SilverSolder> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Tin(0.50), Silver(0.50)].into(),
            num_ingots: 1,
            max_ingots: 25,
        }
    }
}

impl AlloyType for SilverSolder {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl super::private::AlloyType for SilverSolder {
    const NAME: &str = "Silver Solder";
    const RANGES: &[BaseMetal<Range>] =
        &[Tin(Range::new(0.50, 0.60)), Silver(Range::new(0.40, 0.50))];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, super::AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Tin(value.next().ok_or(InvalidConstituentAmounts)?),
            Silver(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_tin, mut seen_silver) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Tin(t) => {
                    Self::check_base_metal(t, 0, seen_tin)?;
                    seen_tin = true;
                    reorder[0] = Tin(*t);
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

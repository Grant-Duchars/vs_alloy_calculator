use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LeadSolder([BaseMetal<i32>; 2]);

impl Default for LeadSolder {
    fn default() -> Self {
        Self([Tin(9), Lead(11)])
    }
}

impl Default for AlloyData<LeadSolder> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Tin(0.45), Lead(0.55)].into(),
            num_ingots: 1,
            max_ingots: 23,
        }
    }
}

impl AlloyType for LeadSolder {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for LeadSolder {
    const NAME: &str = alloy_names::LEAD_SOLDER;
    const RANGES: &[BaseMetal<Range>] =
        &[Tin(Range::new(0.45, 0.55)), Lead(Range::new(0.45, 0.55))];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Tin(value.next().ok_or(InvalidConstituentAmounts)?),
            Lead(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_tin, mut seen_lead) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Tin(t) => {
                    Self::check_base_metal(t, 0, seen_tin)?;
                    seen_tin = true;
                    reorder[0] = Tin(*t);
                }
                Lead(l) => {
                    Self::check_base_metal(l, 1, seen_lead)?;
                    seen_lead = true;
                    reorder[1] = Lead(*l);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

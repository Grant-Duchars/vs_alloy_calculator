use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Molybdochalkos([BaseMetal<i32>; 2]);

impl Default for Molybdochalkos {
    fn default() -> Self {
        Self([Lead(18), Copper(2)])
    }
}

impl Default for Alloy<Molybdochalkos> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Lead(0.92), Copper(0.08)].into(),
            num_ingots: 1,
            max_ingots: 20,
        }
    }
}

impl AlloyType for Molybdochalkos {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for Molybdochalkos {
    const NAME: &str = "Molybdochalkos";
    const RANGES: &[BaseMetal<Range>] =
        &[Lead(Range::new(0.88, 0.92)), Copper(Range::new(0.08, 0.12))];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Lead(value.next().ok_or(InvalidConstituentAmounts)?),
            Copper(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_lead, mut seen_copper) = (false, false);
        let mut reorder = [Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Lead(l) => {
                    Self::check_base_metal(l, 0, seen_lead)?;
                    seen_lead = true;
                    reorder[0] = Lead(*l);
                }
                Copper(c) => {
                    Self::check_base_metal(c, 1, seen_copper)?;
                    seen_copper = true;
                    reorder[1] = Copper(*c);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

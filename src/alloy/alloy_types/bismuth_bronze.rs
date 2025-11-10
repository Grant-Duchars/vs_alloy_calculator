use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BismuthBronze([BaseMetal<i32>; 3]);

impl Default for BismuthBronze {
    fn default() -> Self {
        Self([Copper(12), Zinc(4), Bismuth(4)])
    }
}

impl Default for AlloyData<BismuthBronze> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Copper(0.60), Zinc(0.20), Bismuth(0.20)].into(),
            num_ingots: 1,
            max_ingots: 21,
        }
    }
}

impl AlloyType for BismuthBronze {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for BismuthBronze {
    const NAME: &str = "Bismuth Bronze";
    const RANGES: &[BaseMetal<Range>] = &[
        Copper(Range::new(0.50, 0.70)),
        Zinc(Range::new(0.20, 0.30)),
        Bismuth(Range::new(0.10, 0.20)),
    ];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Copper(value.next().ok_or(InvalidConstituentAmounts)?),
            Zinc(value.next().ok_or(InvalidConstituentAmounts)?),
            Bismuth(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_copper, mut seen_zinc, mut seen_bismuth) = (false, false, false);
        let mut reorder = [Copper(0.0), Copper(0.0), Copper(0.0)];
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
                Bismuth(b) => {
                    Self::check_base_metal(b, 2, seen_bismuth)?;
                    seen_bismuth = true;
                    reorder[2] = Bismuth(*b);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

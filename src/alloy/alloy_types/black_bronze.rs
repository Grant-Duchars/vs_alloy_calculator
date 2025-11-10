use super::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BlackBronze([BaseMetal<i32>; 3]);

impl Default for BlackBronze {
    fn default() -> Self {
        Self([Copper(18), Gold(1), Silver(1)])
    }
}

impl Default for AlloyData<BlackBronze> {
    fn default() -> Self {
        Self {
            alloy_type: Default::default(),
            percentages: [Copper(0.84), Gold(0.08), Silver(0.08)].into(),
            num_ingots: 1,
            max_ingots: 15,
        }
    }
}

impl AlloyType for BlackBronze {
    fn nuggets(&self) -> &[BaseMetal<i32>] {
        &self.0
    }
}

impl private::AlloyType for BlackBronze {
    const NAME: &str = "Black Bronze";
    const RANGES: &[BaseMetal<Range>] = &[
        Copper(Range::new(0.68, 0.84)),
        Gold(Range::new(0.08, 0.16)),
        Silver(Range::new(0.08, 0.16)),
    ];

    fn try_from_vec(value: Vec<i32>) -> Result<Self, AlloyError> {
        let mut value = value.into_iter();
        Ok(Self([
            Copper(value.next().ok_or(InvalidConstituentAmounts)?),
            Gold(value.next().ok_or(InvalidConstituentAmounts)?),
            Silver(value.next().ok_or(InvalidConstituentAmounts)?),
        ]))
    }

    fn check_own_ranges_contains(
        percentages: &[BaseMetal<f32>],
    ) -> Result<Box<[BaseMetal<f32>]>, AlloyError> {
        let (mut seen_copper, mut seen_gold, mut seen_silver) = (false, false, false);
        let mut reorder = [Copper(0.0), Copper(0.0), Copper(0.0)];
        for p in percentages {
            match p {
                Copper(c) => {
                    Self::check_base_metal(c, 0, seen_copper)?;
                    seen_copper = true;
                    reorder[0] = Copper(*c);
                }
                Gold(g) => {
                    Self::check_base_metal(g, 1, seen_gold)?;
                    seen_gold = true;
                    reorder[1] = Gold(*g);
                }
                Silver(s) => {
                    Self::check_base_metal(s, 2, seen_silver)?;
                    seen_silver = true;
                    reorder[2] = Silver(*s);
                }
                _ => return Err(InvalidBaseMetals),
            }
        }
        Ok(reorder.into())
    }
}

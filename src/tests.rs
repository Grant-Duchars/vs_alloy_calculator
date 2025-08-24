#[cfg(test)]
mod tin_bronze_tests {
    use crate::AlloyError::*;
    use crate::prelude::*;

    const MAX_COPPER: [BaseMetal<f32>; 2] = [Copper(0.92), Tin(0.08)];
    const MIN_COPPER: [BaseMetal<f32>; 2] = [Copper(0.88), Tin(0.12)];

    #[test]
    fn test_default() {
        let alloy = Alloy::<TinBronze>::try_new(MAX_COPPER, 1).unwrap();
        assert_eq!(Alloy::<TinBronze>::default(), alloy);
    }

    #[test]
    fn test_any_order_base_metals() {
        let alloy1 = Alloy::<TinBronze>::try_new([Copper(0.92), Tin(0.08)], 1).unwrap();
        let alloy2 = Alloy::<TinBronze>::try_new([Tin(0.08), Copper(0.92)], 1).unwrap();
        assert_eq!(alloy1, alloy2);
    }

    #[test]
    fn test_one_ingot_max_copper() {
        let alloy = Alloy::<TinBronze>::try_new(MAX_COPPER, 1).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(18), nuggets[0]);
        assert_eq!(Tin(2), nuggets[1]);
    }

    #[test]
    fn test_one_ingot_min_copper() {
        let alloy = Alloy::<TinBronze>::try_new(MIN_COPPER, 1).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(17), nuggets[0]);
        assert_eq!(Tin(3), nuggets[1]);
    }

    #[test]
    fn test_ten_ingots_max_copper() {
        let alloy = Alloy::<TinBronze>::try_new(MAX_COPPER, 10).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(184), nuggets[0]);
        assert_eq!(Tin(16), nuggets[1]);
    }

    #[test]
    fn test_ten_ingots_min_copper() {
        let alloy = Alloy::<TinBronze>::try_new(MIN_COPPER, 10).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(176), nuggets[0]);
        assert_eq!(Tin(24), nuggets[1]);
    }

    #[test]
    fn test_invalid_percentages() {
        assert!(
            Alloy::<TinBronze>::try_new([Copper(0.08), Tin(0.92)], 1)
                .is_err_and(|e| e == InvalidPercentages)
        )
    }

    #[test]
    fn test_invalid_base_metals() {
        assert!(
            Alloy::<TinBronze>::try_new([Lead(0.92), Copper(0.08)], 1)
                .is_err_and(|e| e == InvalidBaseMetals)
        )
    }

    #[test]
    fn test_too_many_ingots() {
        assert!(Alloy::<TinBronze>::try_new(MAX_COPPER, 30).is_err_and(|e| e == TooManyIngots));
    }

    #[test]
    fn test_too_few_ingots() {
        assert!(Alloy::<TinBronze>::try_new(MAX_COPPER, 0).is_err_and(|e| e == TooFewIngots))
    }
}

#[cfg(test)]
mod bismuth_bronze_tests {
    use crate::prelude::*;

    const MAX_COPPER: [BaseMetal<f32>; 3] = [Copper(0.70), Zinc(0.20), Bismuth(0.10)];
    const MIN_COPPER: [BaseMetal<f32>; 3] = [Copper(0.50), Zinc(0.30), Bismuth(0.20)];

    #[test]
    fn test_default() {
        let alloy =
            Alloy::<BismuthBronze>::try_new([Copper(0.60), Zinc(0.20), Bismuth(0.20)], 1).unwrap();
        assert_eq!(Alloy::<BismuthBronze>::default(), alloy);
    }

    #[test]
    fn test_any_order_base_metals() {
        let alloy1 =
            Alloy::<BismuthBronze>::try_new([Copper(0.70), Zinc(0.20), Bismuth(0.10)], 1).unwrap();
        let alloy2 =
            Alloy::<BismuthBronze>::try_new([Bismuth(0.10), Copper(0.70), Zinc(0.20)], 1).unwrap();
        assert_eq!(alloy1, alloy2);
    }

    #[test]
    fn test_one_ingot_max_copper() {
        let alloy = Alloy::<BismuthBronze>::try_new(MAX_COPPER, 1).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(14), nuggets[0]);
        assert_eq!(Zinc(4), nuggets[1]);
        assert_eq!(Bismuth(2), nuggets[2]);
    }

    #[test]
    fn test_one_ingot_min_copper() {
        let alloy = Alloy::<BismuthBronze>::try_new(MIN_COPPER, 1).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(10), nuggets[0]);
        assert_eq!(Zinc(6), nuggets[1]);
        assert_eq!(Bismuth(4), nuggets[2]);
    }

    #[test]
    fn test_ten_ingots_max_copper() {
        let alloy = Alloy::<BismuthBronze>::try_new(MAX_COPPER, 10).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(140), nuggets[0]);
        assert_eq!(Zinc(40), nuggets[1]);
        assert_eq!(Bismuth(20), nuggets[2]);
    }

    #[test]
    fn test_ten_ingots_min_copper() {
        let alloy = Alloy::<BismuthBronze>::try_new(MIN_COPPER, 10).unwrap();
        let nuggets = alloy.constituents().nuggets();
        assert_eq!(Copper(100), nuggets[0]);
        assert_eq!(Zinc(60), nuggets[1]);
        assert_eq!(Bismuth(40), nuggets[2]);
    }
}

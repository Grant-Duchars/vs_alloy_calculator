# Vintage Story Alloy Calculator Library

This is an overly engineered library for working with the different alloys from the game [Vintage Story](https://www.vintagestory.at/). This project was mainly created for me to learn and experiment more with Rust and its traits system as well as learning how to make a useful library.
A usable implementation can "soon" be found at my website [grant.duchars.dev/alloy-calculator](https://grant.duchars.dev/alloy-calculator)

## Example

```rust
use vs_alloy_calculator::prelude::*;

let input = { // Mock getting input from user
    // Show the user the valid ranges for the alloy
    let ranges = Alloy::<BismuthBronze>::percentage_ranges();
    // Get the input back from the user
    (Box::from([Copper(0.60), Zinc(0.20), Bismuth(0.20)]), 13)
};

let alloy = Alloy::<BismuthBronze>::try_new(input.0, input.1).expect("should be valid");
assert_eq!(
    &[Copper(156), Zinc(52), Bismuth(52)],
    alloy.constituents().nuggets(),
);
```

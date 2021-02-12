use std::cmp;

const SMALL_COST: u32 = 3;
const MEDIUM_COST: u32 = 8;
const LARGE_COST: u32 = 15;
const XL_COST: u32 = 25;

//Dimensions of a parcel (x, y, z)
struct Dimensions(u32, u32, u32);

//Types of parcel
#[derive(Debug)]
enum Parcel {
    Small,
    Medium,
    Large,
    XL,
}

impl Parcel {
    //Create a new parcel given its dimensions
    fn new(Dimensions(x, y, z): Dimensions) -> Parcel{
        let max_dim = cmp::max(cmp::max(x, y), z);
        if max_dim < 10 {
            Parcel::Small
        } else if max_dim < 50 {
            Parcel::Medium
        } else if max_dim < 100 {
            Parcel::Large
        } else {
            Parcel::XL
        }
    }

    //Return the cost of the parcel
    fn get_cost(&self) -> u32 {
        match self {
            Parcel::Small => SMALL_COST,
            Parcel::Medium => MEDIUM_COST,
            Parcel::Large => LARGE_COST,
            Parcel::XL => XL_COST,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_PARCEL: Parcel = Parcel::Small;
    const MEDIUM_PARCEL: Parcel = Parcel::Medium;

    #[test]
    fn single_small_parcel_cost_correct() {
        assert_eq!(SMALL_PARCEL.get_cost(), SMALL_COST);
    }

    #[test]
    fn single_medium_parcel_cost_correct() {
        assert_eq!(MEDIUM_PARCEL.get_cost(), MEDIUM_COST);
    }

    #[test]
    fn small_box_becomes_small_parcel() -> Result<(), String> {
        let small_box = Parcel::new(Dimensions (1, 1, 1));
        match small_box {
            Parcel::Small => Ok(()),
            _ => Err(String::from(format!("Produced incorrect parcel type: {:?}", small_box))),
        }
    }

    #[test]
    fn parcel_from_box_uses_largest_dimension() -> Result<(), String> {
        let long_box = Parcel::new(Dimensions (1, 25, 50));
        match long_box {
            Parcel::Large => Ok(()),
            _ => Err(String::from(format!("Produced incorrect parcel type: {:?}", long_box))),
        }
    }
}

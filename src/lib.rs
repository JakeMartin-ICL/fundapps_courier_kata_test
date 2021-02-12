const SMALL_COST: u32 = 3;
const MEDIUM_COST: u32 = 8;
const LARGE_COST: u32 = 15;
const XL_COST: u32 = 25;

//Types of parcel
enum Parcel {
    Small,
    Medium,
    Large,
    XL,
}

impl Parcel {
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
}

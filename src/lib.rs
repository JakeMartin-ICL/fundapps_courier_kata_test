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
            Parcel::Small => 3,
            Parcel::Medium => 8,
            Parcel::Large => 15,
            Parcel::XL => 25,
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
        assert_eq!(SMALL_PARCEL.get_cost(), 3);
    }

    #[test]
    fn single_medium_parcel_cost_correct() {
        assert_eq!(MEDIUM_PARCEL.get_cost(), 8);
    }
}
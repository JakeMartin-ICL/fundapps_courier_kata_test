
//Types of parcel
enum Parcel {
    Small,
}

impl Parcel {
    //Return the cost of the parcel
    fn get_cost(&self) -> u32 {
        match self {
            Parcel::Small => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_PARCEL: Parcel = Parcel::Small;

    #[test]
    fn single_small_parcel_cost_correct() {
        assert_eq!(SMALL_PARCEL.get_cost(), 3);
    }
}
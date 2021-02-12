#![feature(exclusive_range_pattern)]
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
    fn new(Dimensions(x, y, z): Dimensions) -> Parcel {
        let max_dim = cmp::max(cmp::max(x, y), z);
        match max_dim {
            0..10 => Parcel::Small,
            10..50 => Parcel::Medium,
            50..100 => Parcel::Large,
            _ => Parcel::XL,
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

    //Produce text string giving name and cost of parcel
    fn display(&self) -> String {
        String::from(match self {
            Parcel::Small => "Small Parcel: $",
            Parcel::Medium => "Medium Parcel: $",
            Parcel::Large => "Large Parcel: $",
            Parcel::XL => "XL Parcel: $",
        }) + &self.get_cost().to_string()
    }
}

//A collection of parcels that form an order
struct Order {
    parcels: Vec<Parcel>,
}

impl Order {
    //Calculate the total cost of the order
    fn calculate_order(&self) -> u32 {
        self.parcels.iter().map(|x| x.get_cost()).sum()
    }

    //Produce text invoice giving itemised list of all parcels and a total cost
    fn produce_invoice(&self) -> String {
        let mut itemised_list = String::new();
        for parcel in &self.parcels {
            itemised_list.push_str(&parcel.display());
            itemised_list.push_str("\n");
        }

        let invoice = format!(
            "{}\nTotal Cost: {}",
            itemised_list,
            self.calculate_order().to_string()
        );
        
        println!("{}", invoice);
        return invoice;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_PARCEL: Parcel = Parcel::Small;
    const MEDIUM_PARCEL: Parcel = Parcel::Medium;
    const LARGE_PARCEL: Parcel = Parcel::Large;
    const XL_PARCEL: Parcel = Parcel::XL;

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
        let small_box = Parcel::new(Dimensions(1, 1, 1));
        match small_box {
            Parcel::Small => Ok(()),
            _ => Err(String::from(format!(
                "Produced incorrect parcel type: {:?}",
                small_box
            ))),
        }
    }

    #[test]
    fn parcel_from_box_uses_largest_dimension() -> Result<(), String> {
        let long_box = Parcel::new(Dimensions(1, 25, 50));
        match long_box {
            Parcel::Large => Ok(()),
            _ => Err(String::from(format!(
                "Produced incorrect parcel type: {:?}",
                long_box
            ))),
        }
    }

    #[test]
    fn order_with_one_small_parcel_cost_correct() {
        let order = Order {
            parcels: vec![SMALL_PARCEL],
        };
        assert_eq!(order.calculate_order(), SMALL_COST)
    }

    #[test]
    fn order_with_one_of_each_parcel_cost_correct() {
        let order = Order {
            parcels: vec![SMALL_PARCEL, MEDIUM_PARCEL, LARGE_PARCEL, XL_PARCEL],
        };
        assert_eq!(
            order.calculate_order(),
            SMALL_COST + MEDIUM_COST + LARGE_COST + XL_COST
        )
    }

    #[test]
    fn parcel_can_produce_text_giving_name_and_cost() {
        assert_eq!(
            SMALL_PARCEL.display(),
            format!("Small Parcel: ${}", &SMALL_COST.to_string())
        )
    }

    #[test]
    fn order_can_produce_itemised_invoice() {
        let order = Order {
            parcels: vec![SMALL_PARCEL, MEDIUM_PARCEL, LARGE_PARCEL, XL_PARCEL],
        };
        assert_eq!(
            order.produce_invoice(),
            format!("Small Parcel: ${}\nMedium Parcel: ${}\nLarge Parcel: ${}\nXL Parcel: ${}\n\nTotal Cost: {}", 
                    &SMALL_COST.to_string(),
                    &MEDIUM_COST.to_string(),
                    &LARGE_COST.to_string(),
                    &XL_COST.to_string(),
                    &order.calculate_order().to_string()))
    }
}

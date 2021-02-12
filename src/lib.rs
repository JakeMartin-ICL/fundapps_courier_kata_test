#![feature(exclusive_range_pattern)]
use std::cmp;

const SMALL_COST: u32 = 3;
const MEDIUM_COST: u32 = 8;
const LARGE_COST: u32 = 15;
const XL_COST: u32 = 25;
const HEAVY_COST: u32 = 50;

const SMALL_WEIGHT_LIM: u32 = 1;
const MEDIUM_WEIGHT_LIM: u32 = 3;
const LARGE_WEIGHT_LIM: u32 = 6;
const XL_WEIGHT_LIM: u32 = 10;
const HEAVY_WEIGHT_LIM: u32 = 50;
const OVERWEIGHT_COST_PER_KG: u32 = 2;
const HEAVY_OVERWEIGHT_COST_PER_KG: u32 = 1;
const SPEEDY_SHIPPING_MULT: u32 = 2;

//Dimensions of a parcel (x, y, z)
struct Dimensions(u32, u32, u32);

//Types of parcel and weight
#[derive(Debug)]
enum Parcel {
    Small(u32),
    Medium(u32),
    Large(u32),
    XL(u32),
    Heavy(u32),
}

impl Parcel {
    //Create a new parcel given its dimensions and weight
    fn new(Dimensions(x, y, z): Dimensions, weight: u32) -> Parcel {
        let max_dim = cmp::max(cmp::max(x, y), z);
        let parcel = match max_dim {
            0..10 => Parcel::Small(weight),
            10..50 => Parcel::Medium(weight),
            50..100 => Parcel::Large(weight),
            _ => Parcel::XL(weight),
        };

        if parcel.get_cost() > HEAVY_COST + Parcel::weight_fee(&weight, HEAVY_WEIGHT_LIM) / 2 {
            return Parcel::Heavy(weight);
        }
        return parcel;
    }

    //Return the cost of the parcel
    fn get_cost(&self) -> u32 {
        match self {
            Parcel::Small(weight) => SMALL_COST + Parcel::weight_fee(&weight, SMALL_WEIGHT_LIM),
            Parcel::Medium(weight) => MEDIUM_COST + Parcel::weight_fee(&weight, MEDIUM_WEIGHT_LIM),
            Parcel::Large(weight) => LARGE_COST + Parcel::weight_fee(&weight, LARGE_WEIGHT_LIM),
            Parcel::XL(weight) => XL_COST + Parcel::weight_fee(&weight, LARGE_WEIGHT_LIM),
            Parcel::Heavy(weight) => HEAVY_COST + Parcel::weight_fee(&weight, HEAVY_WEIGHT_LIM) / 2,
        }
    }

    //Calculate any additional cost for parcel being over weight limit
    fn weight_fee(&weight: &u32, weight_limit: u32) -> u32{
        if weight > weight_limit {OVERWEIGHT_COST_PER_KG * (weight - weight_limit)} else {0}
    }

    //Produce text string giving name and cost of parcel
    fn display(&self) -> String {
        String::from(match self {
            Parcel::Small(_) => "Small Parcel: $",
            Parcel::Medium(_) => "Medium Parcel: $",
            Parcel::Large(_) => "Large Parcel: $",
            Parcel::XL(_) => "XL Parcel: $",
            Parcel::Heavy(_) => "Heavy Parcel: $",
        }) + &self.get_cost().to_string()
    }
}

//A collection of parcels that form an order
struct Order {
    parcels: Vec<Parcel>,
    speedy_shipping: bool,
}

impl Order {
    //Produce new order given parcel vector and speedy_shipping bool (true => speedy selected)
    fn new(parcels: Vec<Parcel>, speedy_shipping: bool) -> Order {
        Order {
            parcels,
            speedy_shipping,
        }
    }

    //Calculate total cost of order
    fn calculate_order(&self) -> u32 {
        self.parcel_cost() * if self.speedy_shipping {SPEEDY_SHIPPING_MULT} else {1} 
    }

    //Calculate sub-total cost of all parcels
    fn parcel_cost(&self) -> u32 {
        self.parcels.iter().map(|x| x.get_cost()).sum()
    }

    //Produce text invoice giving itemised list of all parcels and a total cost
    fn produce_invoice(&self) -> String {
        let mut itemised_list = String::new();
        for parcel in &self.parcels {
            itemised_list.push_str(&parcel.display());
            itemised_list.push_str("\n");
        }

        if self.speedy_shipping {
            itemised_list.push_str(&format!("Speedy Shipping: ${}\n", self.parcel_cost()));
        }

        let invoice = format!(
            "{}\nTotal Cost: ${}",
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

    const SMALL_PARCEL: Parcel = Parcel::Small(1);
    const MEDIUM_PARCEL: Parcel = Parcel::Medium(1);
    const LARGE_PARCEL: Parcel = Parcel::Large(1);
    const XL_PARCEL: Parcel = Parcel::XL(1);

    //Part 1 tests --------------------------------------------------------------

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
        let small_box = Parcel::new(Dimensions(1, 1, 1), 1);
        match small_box {
            Parcel::Small(_) => Ok(()),
            _ => Err(String::from(format!(
                "Produced incorrect parcel type: {:?}",
                small_box
            ))),
        }
    }

    #[test]
    fn parcel_from_box_uses_largest_dimension() -> Result<(), String> {
        let long_box = Parcel::new(Dimensions(1, 25, 50), 1);
        match long_box {
            Parcel::Large(_) => Ok(()),
            _ => Err(String::from(format!(
                "Produced incorrect parcel type: {:?}",
                long_box
            ))),
        }
    }

    #[test]
    fn order_with_one_small_parcel_cost_correct() {
        let order = Order::new(vec![SMALL_PARCEL], false);
        assert_eq!(order.calculate_order(), SMALL_COST)
    }

    #[test]
    fn order_with_one_of_each_parcel_cost_correct() {
        let order = Order::new(
            vec![SMALL_PARCEL, MEDIUM_PARCEL, LARGE_PARCEL, XL_PARCEL],
            false,
        );
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
        let order = Order::new(
            vec![SMALL_PARCEL, MEDIUM_PARCEL, LARGE_PARCEL, XL_PARCEL],
            false,
        );
        assert_eq!(
            order.produce_invoice(),
            format!("Small Parcel: ${}\nMedium Parcel: ${}\nLarge Parcel: ${}\nXL Parcel: ${}\n\nTotal Cost: ${}", 
                    &SMALL_COST.to_string(),
                    &MEDIUM_COST.to_string(),
                    &LARGE_COST.to_string(),
                    &XL_COST.to_string(),
                    &order.calculate_order().to_string()))
    }

    //Part 2 tests --------------------------------------------------------------

    #[test]
    fn speedy_shipping_doubles_price() {
        let order = Order::new(
            vec![SMALL_PARCEL, MEDIUM_PARCEL, LARGE_PARCEL, XL_PARCEL],
            true,
        );
        assert_eq!(
            order.calculate_order(),
            2 * (SMALL_COST + MEDIUM_COST + LARGE_COST + XL_COST)
        )
    }

    #[test]
    fn speedy_shipping_invoice_format() {
        let order = Order::new(
            vec![SMALL_PARCEL, MEDIUM_PARCEL, LARGE_PARCEL, XL_PARCEL],
            true,
        );
        assert_eq!(
            order.produce_invoice(),
            format!("Small Parcel: ${}\nMedium Parcel: ${}\nLarge Parcel: ${}\nXL Parcel: ${}\nSpeedy Shipping: ${}\n\nTotal Cost: ${}", 
                    &SMALL_COST.to_string(),
                    &MEDIUM_COST.to_string(),
                    &LARGE_COST.to_string(),
                    &XL_COST.to_string(),
                    (SMALL_COST + MEDIUM_COST + LARGE_COST + XL_COST).to_string(),
                    &order.calculate_order().to_string()))
    }

    //Part 3 tests --------------------------------------------------------------

    #[test]
    fn overweight_small_parcel_cost() {
        let parcel = Parcel::new(Dimensions(1, 1, 1), 3);
        assert_eq!(parcel.get_cost(), 7)
    }

    #[test]
    fn overweight_large_parcel_cost() {
        let parcel = Parcel::new(Dimensions(1, 25, 50), 10);
        assert_eq!(parcel.get_cost(), 23)
    }

    #[test]
    fn overweight_invoice_correct() {
        let order = Order::new(
            vec![
                Parcel::new(Dimensions(1, 1, 1), 3),
                Parcel::new(Dimensions(1, 25, 50), 10),
            ],
            false,
        );
        assert_eq!(
            order.produce_invoice(),
            "Small Parcel: $7\nLarge Parcel: $23\n\nTotal Cost: $30"
        )
    }

    //Part 4 tests --------------------------------------------------------------

    #[test]
    fn parcel_is_heavy_if_cheaper() -> Result<(), String> {
        let overweight_small = Parcel::new(Dimensions(1, 1, 1), 24);
        let heavy_small = Parcel::new(Dimensions(1, 1, 1), 25);

        match overweight_small {
            Parcel::Small(_) => Ok(()),
            _ => Err(String::from(format!(
                "Produced incorrect parcel type: {:?},should be Small",
                overweight_small
            ))),
        };
        match heavy_small {
            Parcel::Heavy(_) => Ok(()),
            _ => Err(String::from(format!(
                "Produced incorrect parcel type: {:?},should be Heavy",
                heavy_small
            ))),
        }
    }

    #[test]
    fn heavy_parcel_price_correct() {
        assert_eq!(Parcel::new(Dimensions(1, 1, 1), 25).get_cost(), 50)
    }

    #[test]
    fn overweight_heavy_parcel_price_correct() {
        assert_eq!(Parcel::new(Dimensions(1, 1, 1), 55).get_cost(), 55)
    }
}

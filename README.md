# fundapps_courier_kata_test

## Usage and technical details
I have created the library using Rust. For details on how to install Rust, visit the [Rust website](https://www.rust-lang.org/tools/install). The tests can be run using `cargo test`
from the project directory. The exposed API would likely consist of `Parcel::new` and `Order::new`, which create new parcels and orders respectively, and `Order::produce_invoice` which
produces the itemised invoice for the order.

## What I have achieved
In the two hours, I successfully created a library that performs the tasks set out in parts 1-4. The code is modular and easily extensible, with good test coverage.

## What I'd like to improve on
I ran out of time to implement part 5. I began laying the framework for this by adding a sort, but I didn't have time to do any more, though I do have ideas on what I could've done next.
For example, turn the list into an iterator and consume parcels while adding discounts to the invoice as necessary - though I was still mulling over some of the finer details on how to ensure the discount was optimal.
At worst, I figured that I could always use recursion to brute-force an optimal solution.

I also would have liked to perform further testing on edge cases and perhaps handled erroneous or float inputs.

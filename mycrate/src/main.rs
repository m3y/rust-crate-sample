extern crate mycrate;

use mycrate::mymodule::fizzbuzz;

fn main() {
    for i in 1..101 {
        println!("{}: {}", i, fizzbuzz(i));
    }
}

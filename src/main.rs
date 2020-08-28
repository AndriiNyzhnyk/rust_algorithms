mod sorting;
mod other;

use crate::sorting::sort;
use crate::other::algorithms;


fn main() {
    println!("Hello, world!");

    algorithms::run_binary_search_arr();
    algorithms::run_binary_search_vec();

    sort::run_selection_sort_vec();
    sort::run_selection_sort_arr();

    let factorial_recursive: u128 = algorithms::factorial_recursive(32);
    println!("{}", factorial_recursive);


    let factorial_loop: u128 = algorithms::factorial_loop(32);
    println!("{}", factorial_loop);
}


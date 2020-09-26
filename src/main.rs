mod sorting;
mod other;

use crate::sorting::sort;
use crate::other::algorithms;


fn main() {
    println!("Hello, world!");

    algorithms::run_binary_search_arr();
    algorithms::run_binary_search_vec();
    algorithms::run_recursive_binary_search_arr();

    algorithms::run_countdown();

    algorithms::run_sum_arr_loop();
    algorithms::run_sum_arr_recursive();

    algorithms::run_figure_out_arr_len_recursive();

    algorithms::run_find_max_item_recursive();

    sort::run_selection_sort_vec();
    sort::run_selection_sort_arr();

    // sort::run_quick_sort_vec();

    algorithms::run_factorial();
}


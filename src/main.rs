fn main() {
    println!("Hello, world!");

    run_binary_search_arr();
    run_binary_search_vec();

    run_selection_sort_vec();
    run_selection_sort_arr();

    let factorial_recursive: u128 = factorial_recursive(32);
    println!("{}", factorial_recursive);


    let factorial_loop: u128 = factorial_loop(32);
    println!("{}", factorial_loop);
}

fn run_binary_search_arr() {

    let arr:[i32; 7] = [1, 15, 22, 43, 168, 500, 502];

    let result_1 = binary_search_arr(&arr, 1);
    assert_eq!(result_1, 0);

    let result_2 = binary_search_arr(&arr, 15);
    assert_eq!(result_2, 1);

    let result_3 = binary_search_arr(&arr, 22);
    assert_eq!(result_3, 2);

    let result_4 = binary_search_arr(&arr, 43);
    assert_eq!(result_4, 3);

    let result_5 = binary_search_arr(&arr, 168);
    assert_eq!(result_5, 4);

    let result_6 = binary_search_arr(&arr, 500);
    assert_eq!(result_6, 5);

    let result_7 = binary_search_arr(&arr, 502);
    assert_eq!(result_7, 6);

    let result_8 = binary_search_arr(&arr, 888);
    assert_eq!(result_8, -1);
}



fn binary_search_arr(arr: &[i32; 7], find: i32) -> i32 {
    let mut min: u32 = 0;
    let mut max: u32 = (arr.len() - 1) as u32;

    while min <= max {
        let mid: u32 = (min + max) / 2;
        let guess: i32 = arr[mid as usize];

        if guess == find {
            return mid as i32;
        } else if guess < find {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }

    return -1;
}

fn run_binary_search_vec() {
    let v: Vec<i32> = vec![1, 15, 22, 43, 168, 500, 502];

    let result_1 = binary_search_vec(&v, 1);
    assert_eq!(result_1, 0);

    let result_2 = binary_search_vec(&v, 15);
    assert_eq!(result_2, 1);

    let result_3 = binary_search_vec(&v, 22);
    assert_eq!(result_3, 2);

    let result_4 = binary_search_vec(&v, 43);
    assert_eq!(result_4, 3);

    let result_5 = binary_search_vec(&v, 168);
    assert_eq!(result_5, 4);

    let result_6 = binary_search_vec(&v, 500);
    assert_eq!(result_6, 5);

    let result_7 = binary_search_vec(&v, 502);
    assert_eq!(result_7, 6);

    let result_8 = binary_search_vec(&v, 888);
    assert_eq!(result_8, -1);
}

fn binary_search_vec(v: &Vec<i32>, find: i32) -> i32 {
    let mut min: u32 = 0;
    let mut max: u32 = (v.len() - 1) as u32;

    while min <= max {
        let mid: u32 = (min + max) / 2;
        let guess: i32 = v[mid as usize];

        if guess == find {
            return mid as i32;
        } else if guess < find {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }

    return -1;
}


fn run_selection_sort_vec() {
    let v: Vec<i32> = vec![0, -10, 10, -4, -5];
    selection_sort_vec(&v);
}


fn selection_sort_vec(v: &Vec<i32>) -> Vec<i32>{
    let mut input_vector: Vec<i32> = v.clone();
    let mut sorted_vector: Vec<i32> = Vec::new();

    for _ in v.iter() {
        let smallest_element_with_index = find_smallest_element_into_vec(&input_vector);
        sorted_vector.push(input_vector[smallest_element_with_index]);
        input_vector.swap_remove(smallest_element_with_index);
    }

    return sorted_vector;
}


fn find_smallest_element_into_vec(v: &Vec<i32>) -> usize {
    let mut smallest: &i32 = &v[0];
    let mut smallest_element_with_index: usize = 0;

    for (pos, elem) in v.iter().enumerate() {
        if elem < smallest {
            smallest = elem;
            smallest_element_with_index = pos;
        }
    }

    return smallest_element_with_index;
}


fn run_selection_sort_arr() {
    let arr: [i32; 5] = [0, -10, 10, -4, -5];
    selection_sort_arr(&arr);
}

// TODO implement shrink(reducing size) for array
fn selection_sort_arr(arr: &[i32; 5]) -> [i32; 5]{
    let input_arr: [i32; 5] = arr.clone();
    let mut sorted_arr: [i32; 5] = arr.clone();
    let mut index: usize = 0;

    for _ in arr.iter() {
        let smallest_element_with_index = find_smallest_element_into_arr(&input_arr);
        sorted_arr[index] = input_arr[smallest_element_with_index];
        index += 1;
        // sorted_vector.push(input_vector[smallest_element_with_index]);
        // input_vector.swap_remove(smallest_element_with_index);
    }

    return sorted_arr;
}

fn find_smallest_element_into_arr(arr:&[i32; 5]) -> usize {
    let mut smallest: &i32 = &arr[0];
    let mut smallest_element_with_index: usize = 0;

    for (pos, elem) in arr.iter().enumerate() {
        if elem < smallest {
            smallest = elem;
            smallest_element_with_index = pos;
        }
    }

    return smallest_element_with_index;
}

fn factorial_recursive(n: u128) -> u128 {
    if n == 1 {
        return n;
    }

    return n * factorial_recursive(n - 1);
}

fn factorial_loop(n: u128) -> u128 {
    let mut counter: u128 = n;
    let mut result: u128 = 1;

    while counter > 1 {
        result *= counter;
        counter -= 1;
    }

    return result;
}
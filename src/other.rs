pub mod algorithms {
    pub fn run_binary_search_arr() {

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

    fn binary_search_arr(arr: &[i32], find: i32) -> i32 {
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



    pub fn run_binary_search_vec() {
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

    pub fn run_recursive_binary_search_arr() {

        let arr:[i32; 7] = [1, 15, 22, 43, 168, 500, 502];

        let result_1 = recursive_binary_search_arr(&arr, 1);
        assert_eq!(result_1, 0);

        let result_2 = recursive_binary_search_arr(&arr, 15);
        assert_eq!(result_2, 1);

        // let result_3 = recursive_binary_search_arr(&arr, 22);
        // assert_eq!(result_3, 2);
        //
        // let result_4 = recursive_binary_search_arr(&arr, 43);
        // assert_eq!(result_4, 3);
        //
        // let result_5 = recursive_binary_search_arr(&arr, 168);
        // assert_eq!(result_5, 4);
        //
        // let result_6 = recursive_binary_search_arr(&arr, 500);
        // assert_eq!(result_6, 5);

        // let result_7 = recursive_binary_search_arr(&arr, 502);
        // assert_eq!(result_7, 6);

        // let result_8 = recursive_binary_search_arr(&arr, 888);
        // assert_eq!(result_8, -1);

    }

    // TODO implement later
    fn recursive_binary_search_arr(arr: &[i32], find: i32) -> i32 {
        let min: u32 = 0;
        let max: u32 = (arr.len() - 1) as u32;

        if min <= max {
            let mid: u32 = (min + max) / 2;
            let guess: i32 = arr[mid as usize];

            if guess == find {
                return mid as i32;
            } else if guess < find {
                // min = mid + 1;
                return recursive_binary_search_arr(&arr[((mid + 1) as usize)..((max + 1) as usize)], find);
            } else {
                // max = mid - 1;
                return recursive_binary_search_arr(&arr[min as usize..(mid - 1 + 1) as usize], find);
            }
        }

        -1
    }

    pub fn run_factorial() {
        let factorial_recursive: u128 = factorial_recursive(32);
        let factorial_loop: u128 = factorial_loop(32);

        assert_eq!(factorial_recursive, factorial_loop);
    }

    // Rust has three kinds of loops: loop, while, and for. Let’s try each one.
    pub fn run_countdown() {
        countdown_loop(5);
        println!("------");

        countdown_while(5);
        println!("------");

        countdown_for(5);
        println!("------");

        countdown_recursive(5);
    }


    fn countdown_loop(count: usize) {
        let mut i = count;

        loop {
            println!("{}", i);
            i -= 1;

            if i <= 0 {
                break;
            }
        }
    }

    fn countdown_while(count: usize) {
        let mut i = count;

        while i > 0 {
            println!("{}", i);

            i -= 1;
        }
    }


    fn countdown_for(count: usize) {
        for i in (1..count + 1).rev() {
            println!("{}", i);
        }
    }

    fn countdown_recursive(count: usize) {
        println!("{}", count);

        if count > 1 {
            return countdown_recursive(count - 1);
        }
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



    pub fn run_sum_arr_loop() {
        let arr: [i32; 4] = [1, 2, 3, 4];

        let result = sum_arr_loop(&arr);
        assert_eq!(result, 10);
    }

    fn sum_arr_loop(arr: &[i32]) -> i32 {
        let mut sum: i32 = 0;

        for element in arr.iter() {
            sum += element;
        }

        return sum;
    }

    pub fn run_sum_arr_recursive() {
        let arr: [i32; 4] = [1, 2, 3, 4];

        let result = sum_arr_recursive(&arr);
        assert_eq!(result, 10);
    }

    fn sum_arr_recursive(arr: &[i32]) -> i32 {
        if arr.len() == 1 {
            return arr[0];
        }

        return arr[0] + sum_arr_recursive(&arr[1..]);
    }



    pub fn run_figure_out_arr_len_recursive() {
        let arr: [i32; 4] = [1, 2, 3, 4];

        let result = figure_out_arr_len_recursive(&arr);
        assert_eq!(result, arr.len());
    }

    fn figure_out_arr_len_recursive(arr: &[i32]) -> usize {
        if arr == [] {
            return 0;
        }

        return 1 + figure_out_arr_len_recursive(&arr[1..]);
    }


    pub fn run_find_max_item_recursive() {
        let arr: [i32; 4] = [1, 2, 3, 4];

        let result = find_max_item_recursive(&arr);
        assert_eq!(result, 4);
    }

    fn find_max_item_recursive(arr: &[i32]) -> i32 {
        if arr.len() == 1 {
            return arr[0];
        }

        let max = find_max_item_recursive(&arr[1..]);

        if arr[0] > max {
            return arr[0];
        }

        return max;
    }

}
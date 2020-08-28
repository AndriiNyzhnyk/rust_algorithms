pub mod sort {
    pub fn run_selection_sort_vec() {
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


    pub fn run_selection_sort_arr() {
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
}
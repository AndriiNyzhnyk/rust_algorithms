fn main() {
    println!("Hello, world!");

    let arr:[i32; 7] = [1, 15, 22, 43, 168, 500, 502];
    let find: i32 = 750;

    let result = binary_search(arr, find);
    println!("{}", result);
}

fn  binary_search(arr: [i32; 7], find: i32) -> u32 {
    let mut min: u32 = 0;
    let mut max: u32 = (arr.len() - 1) as u32;

    // println!("{}", min);
    // println!("{}", max);


    while min <= max {
        let mid: u32 = (min + max) / 2;
        let guess = arr[mid as usize];

        if guess == find {
            return mid;
        } else if guess < find {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }

    return 0;

}

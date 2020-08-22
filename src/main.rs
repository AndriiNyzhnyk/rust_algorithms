fn main() {
    println!("Hello, world!");

    let arr:[i32; 7] = [1, 15, 22, 43, 168, 500, 502];
    let find: i32 = 750;

    let result = binary_search(arr, find);
    println!("{}", result);
}

fn binary_search(arr: [i32; 7], find: i32) -> i32 {
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

fn binary_search(mut array: Vec<i32>, value: i32) -> u64{
    array.sort();


    let mut low: u64 = 0;
    let mut high: u64 = (array.len() as u64) - 1;
    let mut mid: u64;


    while low <= high {
        mid = (low + high) / 2;

        if array[mid as usize] == value { return mid;}

        if array[mid as usize] < value
        { low = mid + 1; } else { high = mid - 1; }
    }

    return u64::MAX;
}

fn main() {
    let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    let value: i32 = 6;

    println!("Value {} at index {} in array", value, binary_search(array, value));
}

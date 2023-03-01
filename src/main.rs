fn main() {
    // Sort integers.
    let mut integers = [100, 1, 76, 24, 23, 35, 99, 15, 1, 2, 0, -2, -5, 1000, -1000];
    println!("Integers: {:?}", integers);
    bubble_sort(&mut integers);
    println!("Sorted Integers: {:?}", integers);

    // Sort strings.
    let mut strings = ["orange", "banana", "cherry", "apple", "strawberry"];
    println!("Strings: {:?}", strings);
    bubble_sort(&mut strings);
    println!("Sorted Strings: {:?}", strings);
}

// Time Complexity: O(N^2)
fn bubble_sort<T: Ord>(array: &mut [T]) {
    let n = array.len();
    for _i in 1..n {
        for j in 0..n - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

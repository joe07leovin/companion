fn main() {
    fn linear_search(haystack: &[i32], needle: i32) -> Option<usize> {
        for (i, number) in haystack.iter().enumerate() {
            if needle == *number {
                return Some(i);
            }
        }
        return None;
    }

    let data = [3, 7, 12, 19, 25, 31]; // sorted
    println!("{:?}", linear_search(&data, 19)); // expect Some(3)
    println!("{:?}", linear_search(&data, 99));
}

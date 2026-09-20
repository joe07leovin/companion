fn main() {
    fn binary_search(haystack: &[i32], needle: i32) -> Option<usize> {
        if haystack.is_empty() {
            return None;
        }
        let mut low: usize = 0;
        let mut high: usize = haystack.len() - 1;
        let mut middle: usize = high / 2;
        while low <= high {
            middle = (low + high) / 2;
            if haystack[middle] == needle {
                return Some(middle);
            }
            if haystack[middle] < needle {
                low = middle + 1;
            } else if haystack[middle] > needle {
                high = middle - 1;
            }
        }
        None
    }
    let data = [3, 7, 12, 19, 25, 31];
    println!("{:?}", binary_search(&data, 19)); // expect Some(3) — middle-ish
    println!("{:?}", binary_search(&data, 12)); // expect Some(2) — exact midpoint, round 1
    println!("{:?}", binary_search(&data, 3)); // expect Some(0) — first element
    println!("{:?}", binary_search(&data, 31)); // expect Some(5) — last element
    println!("{:?}", binary_search(&data, 99)); // expect None — bigger than everything
    println!("{:?}", binary_search(&data, 1)); // expect None — smaller than everything
    println!("{:?}", binary_search(&[], 5));
}

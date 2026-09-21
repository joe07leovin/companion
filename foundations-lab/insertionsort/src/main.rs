fn main() {
    fn insertion_sort(unsorted: &mut [i32]) {
        for i in 1..unsorted.len() {
            let key = unsorted[i];
            let mut j = i;
            println!("{:?}  i={} key={} j={}", unsorted, i, key, j);
            while j > 0 && unsorted[j - 1] > key {
                unsorted[j] = unsorted[j - 1];
                println!("  shifted: {:?}", unsorted);
                j = j - 1;
            }
            unsorted[j] = key;
        }
    }
    let mut a = [5, 2, 9, 1, 7];
    insertion_sort(&mut a);
    println!("sorted {:?}", a);
}

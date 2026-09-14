use std::time::Instant;
fn main() {
    for size in [100usize, 1_000, 10_000, 100_000] {
        let mut array = Vec::new();
        let t = Instant::now();
        for n in 0..size {
            array.push(n);
        }
        //f64 float
        let per = t.elapsed().as_nanos() as f64 / size as f64;
        println!("{size}:{per} ns/push");
    }
}

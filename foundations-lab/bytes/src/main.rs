fn main() {
    for n in 0..16 {
        println!("Decimal value: {}", n);
        println!("Hexadecimal value: {:x}", n);
        println!("Hexadecimal with prefix: {:#x}", n);
        println!("Binary value: {:b}", n);
        println!("Binary value padded: {:08b}", n);
    }
    let a: u8 = "255".parse().unwrap();
    let b = a + 1;
    println!("{b}");
}

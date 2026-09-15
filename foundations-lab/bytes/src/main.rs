fn main() {
    for n in 0..16 {
        println!("Decimal value: {}", n);
        println!("Hexadecimal value: {:x}", n);
        println!("Hexadecimal with prefix: {:#x}", n);
        println!("Binary value: {:b}", n);
        println!("Binary value padded: {:08b}", n);
    }
    for n in 0..255 {
        println!("Decimal value: {}", n);
        println!("Hexadecimal value: {:x}", n);
        println!("Hexadecimal with prefix: {:#x}", n);
        println!("Binary value: {:b}", n);
        println!("Binary value padded: {:08b}", n);
    }

    for n in 0..256 {
        println!("Decimal value: {}", n);
        println!("Hexadecimal value: {:x}", n);
        println!("Hexadecimal with prefix: {:#x}", n);
        println!("Binary value: {:b}", n);
        println!("Binary value padded: {:08b}", n);
    }
}

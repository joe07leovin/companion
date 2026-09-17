fn main() {
    let stack_array = [10i32, 20, 30, 40];
    println!("{:p}", &stack_array[0]);
    println!("{:p}", &stack_array[1]);
    println!("{:p}", &stack_array[2]);
    let heap_value = Box::new(42i32);
    println!("stack: {:p}", &stack_array);
    println!("heap: {:p}", heap_value);
}

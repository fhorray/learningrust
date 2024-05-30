fn main() {
    /*
    Integer data types:

    Rust has signed ( + and - ) and unsigned (only +) types of different sizes
    Signed integer data types can be: i8, i16, i32, i64, i128.
    Unsigned integer data types can be: u8, u16, u32, u64, u128.
    */
    let a: i32 = -132;
    let b: u64 = 132;
    println!("Signed int: {}", a);
    println!("Unsigned int: {}", b);

    println!("///////////////////////");
    // Floating point data types
    let c: f32 = 3.14;
    let d: f64 = 132.25;
    println!("Float 32: {}", c);
    println!("Float 64: {}", d);

    println!("///////////////////////");

    // Boolean data type
    let is_snowing: bool = true;
    println!("\nIs it isnowing? {}", is_snowing);

    // Character data type
    let e: char = 'A';
    println!("First letter of the alphabet: {}", e);

    println!("///////////////////////");

    // Array data type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let names: [&str; 4] = ["John", "Maria", "Peter", "Saabrina"];
    let letters: [char; 3] = ['A', 'B', 'C'];
    println!("\nNumbers Array: {:?}", numbers);
    println!("Names Array: {:?}", names);
    println!("Letters Array: {:?}", letters);

    println!("1st name: {:?}", names[0]);
    println!("2nd name: {:?}", names[1]);
    println!("3rd name: {:?}", names[2]);

    println!("///////////////////////");

    // Tuple data type
    let francy: (String, i32, bool) = ("Francy".to_string(), 25, true);
    println!("\nLetters Array: {:?}", francy);

    //          you can omit it!
    let ninja: (String, i32, bool) = ("Itachi Uchiha".to_string(), 25, false);
    println!("Ninja Information:");
    println!(
        "Name: {:?}, HP: {:?}, Has Family? {:?}",
        ninja.0, ninja.1, ninja.2
    );

    println!("///////////////////////");

    // Slice data type
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("\nNumber Slices: {:?}", number_slices);

    // String vs str
    // Data types by default in Rust is imutable.
    // To make it mutable, you need to add the keyword mut

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!"); // ERROR IF: without the mut word it wont work
    println!("Stone Cold Says: {}", stone_cold);


    // String data type

    // Reference data type

    // Pointer data type

    // Function data type

    // Closure data type

    // Option data type

    // Result data type

    // Enum data type

    // Struct data type
}

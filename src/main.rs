use eight_booleans::ByteBool;

fn main() {
    println!("=== ByteBool Operators Usage Example ===\n");

    // Create a new ByteBool instance (starts with all bits set to 0)
    let mut bits = ByteBool::default();
    // let mut bits = ByteBool::<u16>::new();
    println!("Initial state:");
    bits.display();
    println!();

    // SET operation: Set individual bits to true
    println!("--- SET Operation ---");
    println!("Setting bit 0 to true:");
    bits.set(0, true);
    bits.display();

    println!("\nSetting bit 3 to true:");
    bits.set(3, true);
    bits.display();
    println!("\nSetting bit 7 to true:");
    bits.set(7, true);
    bits.display();
    println!();

    // READ operation: Read individual bit values
    println!("--- READ Operation ---");
    for i in 0..=7 {
        let value = bits.read(i);
        println!("Bit {} is {}", i, if value { "1 (true)" } else { "0 (false)" });
    }
    println!();

    // TOGGLE operation: Flip bit values (1 -> 0, 0 -> 1)
    println!("--- TOGGLE Operation ---");
    println!("Before toggle:");
    bits.display();

    println!("\nToggling bit 0:");
    bits.toggle(0);
    bits.display();

    println!("\nToggling bit 3:");
    bits.toggle(3);
    bits.display();
    println!();

    // CLEAR operation: Set bits to false (0)
    println!("--- CLEAR Operation ---");
    println!("Before clearing:");
    bits.display();

    println!("\nClearing all bits:");
    bits.clear();
    bits.display();



    println!("=== Example Complete ===");
    println!("<<End>>")
}

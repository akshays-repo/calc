pub fn run() {
    // print
    println!("Hello this is run");

    // formatting
    println!("akshay is {}", 1);
    println!("my name is {} and i am from {}", "Akshay", "Pluto");

    // Posistonal parameter
    println!(
        "my name is {0} and i am from {1} and {0} like to code",
        "Akshay", "Pluto"
    );
    // Named Argument
    println!(
        "my name is {name} and i am from {planet}",
        name = "Akshay",
        planet = "Pluto"
    );

    // Place holder
    print!("Binary: {:b} Hex: {:x} Octol: {:o}", 10, 1232, 10);
}

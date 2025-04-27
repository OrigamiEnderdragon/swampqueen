use swampqueen_core::dice::{roll_die, roll_many_from_str};
use swampqueen_core::location::Location;

fn main() {
    println!("Let's roll some dice!");
    // Roll dice. Store the RESULT of "roll_die" in a variable called "our_result".
    let our_result = roll_die(6);
    // ...print the result.
    println!("{our_result}");

    println!("I cast... LAME FIREBALL!!!!!");
    println!("It onwy does 3d6 damage :c");

    let die_1 = roll_die(6);
    let die_2 = roll_die(6);
    let die_3 = roll_die(6);

    let damage = die_1 + die_2 + die_3;
    println!("I rolled a {die_1}, a {die_2}, and a {die_3}...");
    println!("I do {damage} damage!!!!!!");

    println!("...");
    println!("Actually...");
    println!("I CAST REAL FIREBALL!!!!!!!!");
    let fireball_result = roll_many_from_str("8d6").unwrap();
    println!("{fireball_result}");

    println!("\n=========Location Test=========\n");
    let testplace = Location::try_load_location("testplace").unwrap();
    println!("{}\n", testplace.paragraph("intro", 0).unwrap());
    println!("{}\n", testplace.paragraph("intro", 1).unwrap());
    println!("{}\n", testplace.paragraph("intro", 2).unwrap());
}

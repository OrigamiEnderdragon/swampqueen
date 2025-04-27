use std::io::{self, Write};

use swampqueen_core::character::{Character, Class, Race, Stat};
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

    println!("\n=========Character Creation Test=========\n");

    print!("Name: ");
    io::stdout().flush().unwrap();
    let mut chosen_name = String::new();
    io::stdin().read_line(&mut chosen_name).unwrap();

    print!("Class: ");
    io::stdout().flush().unwrap();
    let mut chosen_class_str = String::new();
    io::stdin().read_line(&mut chosen_class_str).unwrap();
    let class: Class = chosen_class_str.trim().try_into().unwrap();

    print!("Race: ");
    io::stdout().flush().unwrap();
    let mut chosen_race_str = String::new();
    io::stdin().read_line(&mut chosen_race_str).unwrap();
    let race: Race = chosen_race_str.trim().try_into().unwrap();

    print!("First stat bonus: ");
    io::stdout().flush().unwrap();
    let mut chosen_stat_1_str = String::new();
    io::stdin().read_line(&mut chosen_stat_1_str).unwrap();
    let stat_1: Stat = chosen_stat_1_str.trim().try_into().unwrap();

    print!("Second stat bonus: ");
    io::stdout().flush().unwrap();
    let mut chosen_stat_2_str = String::new();
    io::stdin().read_line(&mut chosen_stat_2_str).unwrap();
    let stat_2: Stat = chosen_stat_2_str.trim().try_into().unwrap();

    let character = Character::new(&chosen_name, class, race, stat_1, stat_2);

    println!("YOUR CHARACTER:");
    dbg!(&character);
}

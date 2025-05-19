use std::io::{self, Write};

use swampqueen_core::{
    character::{Character, Class, Race, Stat},
    dice::{roll_die, roll_many_from_str},
    location::Location,
    ui::{input, select},
};

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

    let chosen_name: String = input("Character name", |input| {
        if input.is_empty() {
            Err("You can't have an empty name!".into())
        } else {
            Ok(String::from(input))
        }
    })
    .unwrap();

    let class = select(
        "Please select your class",
        &[
            Class::Hunter,
            Class::Warden,
            Class::Bastion,
            Class::Soothsayer,
            Class::Trespasser,
        ],
    )
    .unwrap();

    let race = select(
        "Please select your race",
        &[Race::InsectoidFae, Race::GoblinoidFae, Race::AlligatorFolk],
    )
    .unwrap();

    let stat_1 = select(
        "Please select your first bonus stat",
        &[
            Stat::Slipperiness,
            Stat::Cunning,
            Stat::Bulk,
            Stat::Backbone,
            Stat::TheSight,
        ],
    )
    .unwrap();

    let stat_2 = select(
        "Please select your second bonus stat",
        &[
            Stat::Slipperiness,
            Stat::Cunning,
            Stat::Bulk,
            Stat::Backbone,
            Stat::TheSight,
        ],
    )
    .unwrap();

    let character = Character::new(&chosen_name, class, race, stat_1, stat_2);

    println!("{character}");
}

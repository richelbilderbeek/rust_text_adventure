use std::io::{self, BufRead};

fn show_room(room_number: i32) {
    if room_number == 1 { show_room_1(); return; }
    if room_number == 2 { show_room_2(); return; }
    if room_number == 3 { show_room_3(); return; }
    if room_number == 4 { show_room_4(); return; }
    println!("You are in an unexpected room.");
    println!("Check your code :-)");
}

fn show_room_1() {
    println!("You are in room 1.");
    println!(" ");
    println!("What do you want to do?");
    println!("1) Go left");
    println!("2) Go right");
}

fn show_room_2() {
    println!("You are in room 2.");
    println!(" ");
    println!("What do you want to do?");
    println!("1) Go up");
    println!("2) Go down");
}

fn show_room_3() {
    println!("You are in room 3.");
    println!("You die.");
    println!(" ");
    println!("GAME OVER");
    std::process::exit(0)
}

fn show_room_4() {
    println!("You are in room 4.");
    println!("You have escaped!");
    println!(" ");
    println!("GAME WON");
    std::process::exit(0)
}

fn process_input(room_number: &mut i32) {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    if *room_number == 1 && input.trim() == "1" { *room_number = 2; return; }
    if *room_number == 1 && input.trim() == "2" { *room_number = 3; return; }
    if *room_number == 2 && input.trim() == "1" { *room_number = 3; return; }
    if *room_number == 2 && input.trim() == "2" { *room_number = 4; return; }
}

fn main() {
    let mut room_number = 1;
    loop {
      show_room(room_number);
      process_input(&mut room_number);
    }
}

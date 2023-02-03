use rand::Rng;
use std::io;




fn main() {
    println!("Rock, Paper, Scissors");
   
    let mut input = String::new();
   
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
   
    let t = ["Rock", "Paper", "Scissors"];


    let player = input.trim().to_string();


    let mut rng = rand::thread_rng();
    let computer = t[rng.gen_range(0..2)];


    println!("\nYou chose: {}", player);
    println!("The computer chose: {}", computer);


    if player == computer {
        println!("Tied")
    } else if player == "Rock" {
        if computer == "Scissors"{
            println!("You Win!")
        } else {
            println!("You Lose")
        }
    } else if player == "Scissors" {
        if computer == "Paper" {
        println!("You Win!")
        } else {
            println!{"You Lose"}
        }
    } else if player == "Paper" {
        if computer == "Rock" {
            println!("You Win!")
        } else {
            println!("You Lose")
        }
    } else {
        println!("That was not a valid play")
    };
}

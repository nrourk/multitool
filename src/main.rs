use clap::Parser;

mod args;
mod calc;

use args::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Calc) => {
            // L'utilisateur a tapé : cargo run -- calc
            maths();
        }
        None => {
            println!("Aucune commande fournie. Tapez --help pour voir les options.");
        }
    }
}

fn maths() {
    let op = calc::operation_con();
    let x = calc::value_x();
    let y = calc::value_y();

    calc::math_operation(op, x, y);
}
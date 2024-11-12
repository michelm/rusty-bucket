use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Operation; add|+, subtract|-, multiply|x, divide|/
    #[arg(short, long)]
    operator: String,

    /// Argument #1; augend|minuend|multiplier|numerator
    #[arg(short = 'x', long)]
    first: i32,

    /// Argument #2; addend|subtrahend|multiplicand|denominator
    #[arg(short = 'y', long)]
    second: i32,
}

fn main() {
    let args = Args::parse();
    let x = args.first;
    let y = args.second;

    match args.operator.trim() {
        "add" | "+" => {
            let result = rust_github_template::add(x, y);
            println!("{} + {} = {}", x, y, result);
        }
        "subtract" | "-" => {
            let result = rust_github_template::subtract(x, y);
            println!("{} - {} = {}", x, y, result);
        }
        "multiply" | "x" => {
            let result = rust_github_template::multiply(x, y);
            println!("{} x {} = {}", x, y, result);
        }
        "divide" | "/" => {
            let result = rust_github_template::divide(x, y).unwrap();
            println!("{} / {} = {}", x, y, result);
        }
        _ => {
            println!("Invalid operator: {}", args.operator);
        }
    }
}

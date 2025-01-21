use clap::Parser;

#[derive(Parser)]
struct Args {
    greetee: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", tta::greet(&args.greetee));
}

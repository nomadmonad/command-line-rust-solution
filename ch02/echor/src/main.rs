use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echor")]
#[command(version = "0.1.0")]
#[command(author = "Ken Youens-Clark <kyclark@gmail.com>")]
#[command(about = "Rust echo")]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    text: String,
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {:?}", args.text);
    }
}

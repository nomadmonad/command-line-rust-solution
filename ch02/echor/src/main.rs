use clap::{Arg, ArgAction, Command};

// #[derive(Parser, Debug)]
// #[command(name = "echor")]
// #[command(version = "0.1.0")]
// #[command(author = "Ken Youens-Clark <kyclark@gmail.com>")]
// #[command(about = "Rust echo")]
// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     text: String,
//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

fn main() {
    // let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {:?}", args.text);
    // }
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .action(ArgAction::Append)
                .required(true),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();
    println!("{:#?}", matches);
    let text: Vec<&str> = matches
        .get_many("text")
        .unwrap()
        .map(String::as_str)
        .collect();
    let omit_new_line: bool = *matches.get_one("omit_newline").unwrap();
    print!(
        "{}{}",
        text.join(" "),
        if omit_new_line { "" } else { "\n" }
    );
}

use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}

pub fn get_args() -> MyResult<()> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust cat")
        .get_matches();

    Ok(Config {
        files: ...,
        number_lines: ...,
        number_nonblank_lines: ...,
    })
}
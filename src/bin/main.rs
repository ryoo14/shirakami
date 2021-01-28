use exitcode;
use shirakami;
use std::env;
use std::process;

fn main() {
    let input_url: Vec<String> = env::args().collect();
    if input_url.len() != 2 {
        println!("Please input 1 url.");
        process::exit(exitcode::DATAERR);
    }

    let short_url = match shirakami::shorten_url(&input_url[1]) {
        Ok(short_url) => {
            println!("{}", short_url);
            process::exit(exitcode::OK);
        }
        Err(e) => {
            println!("{}", e);
            process::exit(exitcode::DATAERR);
        }
    };
}

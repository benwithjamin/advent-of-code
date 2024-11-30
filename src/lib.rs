use std::{
    fs::File,
    io::{copy, BufRead, BufReader, Lines},
    path::Path,
};

use reqwest::blocking::Client;

pub fn run_on_challenge_input<F>(year: u32, day: u32, mut func: F)
where
    F: FnMut(&str),
{
    let input_file = get_input_file(year, day);

    let path = Path::new(&input_file);
    let working_directory = std::env::current_dir().expect("Error getting current directory");

    let resolved_path = working_directory.join(path);

    if !resolved_path.exists() {
        if let Err(error) = fetch_challenge_input(year, day) {
            println!("Error: {}", error);
            std::process::exit(-1);
        }
    }

    match read_challenge_input(&resolved_path) {
        Ok(lines) => {
            for line in lines.flatten() {
                func(&line);
            }
        }
        Err(error) => {
            println!("Error reading challenge input: {}", error)
        }
    }
}

fn get_input_file(year: u32, day: u32) -> String {
    format!("input/{}-{}.txt", year, day)
}

fn read_challenge_input<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn fetch_challenge_input(
    year: u32,
    day: u32,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use dotenv::dotenv;

    dotenv().ok();

    let session_cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE must be set");

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    println!("{}", url);

    let client = Client::new();
    let response = client.get(url)
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.1")
        .header("cookie", session_cookie)
        .send()?;

    if response.status().is_success() {
        let mut file = File::create(&get_input_file(year, day))?;
        let content = response.bytes()?;

        copy(&mut content.as_ref(), &mut file)?;
    } else {
        eprintln!(
            "Failed to download challenge input for day {} ({}). HTTP status: {}",
            day,
            year,
            response.status()
        );
        eprintln!("{:?}", response);
    }

    Ok(())
}

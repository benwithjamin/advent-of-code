use std::{
    fs::File,
    io::{copy, BufRead, BufReader, Lines, Read},
    path::{Path, PathBuf},
};

use reqwest::blocking::Client;

pub fn get_challenge_input_as_str(year: u64, day: u64) -> std::io::Result<String> {
    let resolved_path = get_input_file(year, day);

    let mut file = File::open(resolved_path)?;
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn run_on_challenge_input_lines_ttb<F>(year: u64, day: u64, mut func: F)
where
    F: FnMut(&str),
{
    let resolved_path = get_input_file(year, day);

    match read_challenge_input(&resolved_path) {
        Ok(lines) => {
            let input_characters: Vec<Vec<char>> = lines
                .filter_map(Result::ok)
                .map(|line| line.chars().collect())
                .collect();

            let rows = input_characters.len();
            let columns = input_characters[0].len();

            let transposed_characters: Vec<Vec<char>> = (0..columns)
                .map(|column| (0..rows).map(|row| input_characters[row][column]).collect())
                .collect();

            for line in transposed_characters.iter() {
                func(&line.iter().collect::<String>());
            }
        }
        Err(error) => println!("Error reading challenge input: {}", error),
    }
}

pub fn run_on_challenge_input_lines<F>(year: u64, day: u64, mut func: F)
where
    F: FnMut(&str),
{
    let resolved_path = get_input_file(year, day);

    match read_challenge_input(&resolved_path) {
        Ok(lines) => {
            for line in lines.flatten() {
                func(&line);
            }
        }
        Err(error) => println!("Error reading challenge input: {}", error),
    }
}

fn get_input_file(year: u64, day: u64) -> PathBuf {
    let input_file = format!("input/{}-{}.txt", year, day);
    let path = Path::new(&input_file);
    let working_directory = std::env::current_dir().expect("Error getting current directory");

    let resolved_path = working_directory.join(path);

    if !resolved_path.exists() {
        if let Err(error) = fetch_challenge_input(year, day) {
            println!("Error: {}", error);
            std::process::exit(-1);
        }
    }

    resolved_path
}

fn read_challenge_input<P>(filename: P) -> std::io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn fetch_challenge_input(
    year: u64,
    day: u64,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    use dotenv::dotenv;

    dotenv().ok();

    let session_cookie = std::env::var("SESSION_COOKIE").expect("SESSION_COOKIE must be set");

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    println!("Downloading challenge input...");

    let client = Client::new();
    let response = client.get(url)
        .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.6 Safari/605.1.1")
        .header("cookie", session_cookie)
        .send()?;

    if response.status().is_success() {
        let mut file = File::create(&format!("input/{}-{}.txt", year, day))?;
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

pub mod utils {
    use regex::Regex;

    pub fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let mut last = 0;
        for m in r.find_iter(text) {
            if last != m.start() {
                result.push(&text[last..m.start()]);
            }
            result.push(m.as_str());
            last = m.start() + m.len();
        }

        if last < text.len() {
            result.push(&text[last..]);
        }

        result
    }
}

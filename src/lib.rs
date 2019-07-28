use std::error::Error;
use std::fs;
use std::process;

use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
      return Err("not enough arguments, see --help");
    }

    if args.contains(&String::from("--help")) {
      println!("kratogrep [SEARCH_WORD] [FILE_PATH] [OPTIONS]");
      println!("Example: kratogrep BOY!!! ./text.txt");
      println!("");
      println!("Options:");
      println!("  --case_insenstive  self explanatory");
      process::exit(0);
    }

    let mut case_insensitive_flag = false;
    if args.contains(&String::from("--case_insensitive")) {
      case_insensitive_flag = true;
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive: case_sensitive || case_insensitive_flag,
    })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    if line.contains("?") {
      let boy = "BOY???";
      let line_part: Vec<&str> = line.split("?").collect();
      println!("{}", [line_part[0], boy].join(" "));
    } else {
      let boy = "BOY!!!";
      let line_part: Vec<&str> = line.split(|c| c == '!' || c == '.' ).collect();
      println!("{}", [line_part[0], boy].join(" "));
    }
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  contents.lines().for_each(|line| {
    if line.contains(query) {
      results.push(line)
    }
  });

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}

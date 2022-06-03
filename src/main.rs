use std::env;
use std::error::Error;
use std::fs;

<<<<<<< HEAD
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
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
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
=======
static N:i32 = 15;


pub fn get_statx()-> String{
    "Arman".to_string()
}

pub fn get_staty<'a>()->&'a str{
  &"Arman"
}

pub fn get_stat()->&'static i32{
    &N
}

pub fn stat_str()->&'static str{
    "hello"
}

static mut M:i32 = 20;

pub fn add_stat(n:i32)->i32 {
    unsafe {
        M += n;
        M
>>>>>>> be2bc5a0fba5638962dbe97bb0b415d71f8bd828
    }

    results
}

<<<<<<< HEAD
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
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
=======
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = get_statx();
        assert_eq!(*x, "Arman".to_string());

        let y = get_staty();
        assert_eq!(y, "Arman");

        let n = get_stat();
        assert_eq!(*n, 15);

        let s = stat_str();
        assert_eq!(s,"hello");

        let m = add_stat(5);
        assert_eq!(m,25);
>>>>>>> be2bc5a0fba5638962dbe97bb0b415d71f8bd828
    }
}

use std::{fs, env};
use std::error::Error;

pub enum CaseSensitive {
    Yes,
    No
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: CaseSensitive,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); 

        let query = match args.next() {
            Some(query) => query,
            None => return Err("No query defined"),
        };

        let filename = match args.next () {
            Some(file) => file,
            None => return Err("No file defined"),
        };

        let case_sensitive = get_case_sensitive();

        Ok(Config { 
            query, 
            filename, 
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    for l in search_line(&config.query, &content, config.case_sensitive) {
        println!("{}", l);
    }

    Ok(())
}

fn get_case_sensitive() -> CaseSensitive {
    match env::var("CASE_INSENSITIVE") {
        Err(_) => CaseSensitive::Yes,
        Ok(_)  => CaseSensitive::No
    }
}

pub fn search_line_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    search_line_core(contents, |l| l.to_lowercase().contains(&query))
}

pub fn search_line_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    search_line_core(contents, |l| l.contains(&query))
}

fn search_line_core<'a, F: Fn(&&str) -> bool>(contents: &'a str, filter_fc: F) -> Vec<&'a str> {
    contents
        .lines()
        .filter(filter_fc)
        .collect()
}

pub fn search_line<'a>(query: &str, contents: &'a str, case_sensitive: CaseSensitive) -> Vec<&'a str> {
    match case_sensitive {
        CaseSensitive::Yes => search_line_case_sensitive(query, contents),
        CaseSensitive::No  => search_line_case_insensitive(query, contents)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_match_one() {
        let query = "duct";
        let contents = "\
        Rust : 
        safe, fast, productive.
        , pick three.
        Duct tape";

        assert_eq!(vec!["safe, fast, productive."], 
                   search_line(query, contents, CaseSensitive::Yes)
                   .iter()
                   .map(|fstr| fstr.trim_start_matches(" "))
                   .collect::<Vec<&str>>())
    }

    #[test]
    fn case_sensitive_match_none() {
        let query = "monomorphization";
        let contents = "\
        Rust : 
        safe, fast, productive.
        , pick three.";

        assert_eq!(Vec::<&str>::new(),
                   search_line(query, contents, CaseSensitive::Yes)
                   .iter()
                   .map(|fstr| fstr.trim_start_matches(" "))
                   .collect::<Vec<&str>>())
    }

    #[test]
    fn case_insensitive_match_one() {
        let query = "trUst";
        let contents = "\
        Rust :
        safe, fast, productive.
        , pick three.
        Trust me";

        assert_eq!(vec!["Trust me"],
                   search_line(query, contents, CaseSensitive::No)
                   .iter()
                   .map(|fstr| fstr.trim_start_matches(" "))
                   .collect::<Vec<&str>>())
    }

    #[test]
    fn case_insensitive_match_two() {
        let query = "rUst";
        let contents = "\
        Rust :
        safe, fast, productive.
        , pick three.
        Trust me";

        assert_eq!(vec!["Rust :", "Trust me"],
                   search_line(query, contents, CaseSensitive::No)
                   .iter()
                   .map(|fstr| fstr.trim_start_matches(" "))
                   .collect::<Vec<&str>>())
    }
}
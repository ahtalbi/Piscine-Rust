pub use chrono::{DateTime, Datelike};
pub use std::collections::HashMap;

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res = HashMap::new();

    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            let login = login.to_owned();
            *res.entry(login).or_insert(0) += 1;
        }
    }

    res
}

pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut res = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
                let year = date.year();
                let week = date.iso_week().week();

                let week = format!("{}-W{}", year, week);

                *res.entry(week).or_insert(0) += 1;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
	    let contents = include_str!("commits.json");
	    let serialized = json::parse(&contents).unwrap();
	    println!("{:?}", commits_per_week(&serialized));
	    println!("{:?}", commits_per_author(&serialized));
    }
}

mod err;

pub use err::{ParseErr, ReadErr};

use json;
use std::{error::Error, fs};

#[derive(Debug, Eq, PartialEq)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub level: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TodoList {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn get_todo(path: &str) -> Result<TodoList, Box<dyn Error>> {
        let todos = fs::read_to_string(path).map_err(|err| ReadErr {
            child_err: Box::new(err),
        })?;

        let prsd_tds = json::parse(&todos).map_err(|err| ParseErr::Malformed(Box::new(err)))?;
        if prsd_tds["tasks"].is_empty() {
            return Err(ParseErr::Empty.into());
        }

        let title = prsd_tds["title"].to_string();
        let mut tasks = vec![];

        for task in prsd_tds["tasks"].members() {
            let id = task["id"].as_u32().unwrap();
            let description = task["description"].to_string();
            let level = task["level"].as_u32().unwrap();

            tasks.push(Task {
                id,
                description,
                level,
            });
        }

        Ok(TodoList { title, tasks })
    }
}
mod err;

use std::{error::Error, fs};
use json::JsonValue;

use err::{ParseErr, ReadErr};

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
        let content = fs::read_to_string(path)
            .map_err(|e| ReadErr {
                child_err: Box::new(e),
            })?;

        let data = json::parse(&content)
            .map_err(|e| ParseErr::Malformed(Box::new(e)))?;

        let title = data["title"]
            .as_str()
            .unwrap_or("")
            .to_string();

        let tasks_json = &data["tasks"];

        if !tasks_json.is_array() || tasks_json.len() == 0 {
            return Err(Box::new(ParseErr::Empty));
        }

        let mut tasks = Vec::new();

        for t in tasks_json.members() {
            tasks.push(Task {
                id: t["id"].as_u32().unwrap_or(0),
                description: t["description"].as_str().unwrap_or("").to_string(),
                level: t["level"].as_u32().unwrap_or(0),
            });
        }

        Ok(TodoList { title, tasks })
    }
}
use chrono::{DateTime, Local};
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use std::{error::Error, time::SystemTime};

pub mod main_test;

#[derive(Debug)]
struct Todo {
    title: String,
    completed: bool,
    created_at: SystemTime,
    completed_at: Option<SystemTime>,
}

struct Todos(Vec<Todo>);

impl Todos {
    fn add(&mut self, title: String) {
        self.0.push(Todo {
            title,
            completed: false,
            created_at: SystemTime::now(),
            completed_at: None,
        });
    }

    fn validate_index(&self, index: usize) -> Result<(), Box<dyn Error>> {
        if index >= self.0.len() {
            let err = "invalid index";
            println!("{}", err);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Index out of bounds",
            )));
        }
        Ok(())
    }

    fn delete_todo(&mut self, index: usize) -> Result<(), Box<dyn Error>> {
        self.validate_index(index)?;
        self.0.remove(index);
        Ok(())
    }
}

fn convert_time(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%d-%m-%Y %H:%M:%S").to_string()
}

fn main() {
    let mut todos = Todos(Vec::new());
    todos.add("Learn Rust".to_string());
    todos.add("Write a blog post".to_string());
    todos.add("Read a book".to_string());
    todos.add("Make dinner".to_string());

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            "#",
            "Completed",
            "Title",
            "Created At",
            "Completed At",
        ]);

    todos.0.iter().enumerate().for_each(|(index, todo)| {
        table.add_row(vec![
            format!("{}", index),
            display_checkmark(todo.completed).to_string(),
            todo.title.clone(),
            convert_time(todo.created_at),
            todo.completed_at
                .map(convert_time)
                .unwrap_or_else(|| "".to_string()),
        ]);
    });

    println!("{table}")
}

fn display_checkmark(completed: bool) -> &'static str {
    if completed {
        "✅"
    } else {
        "❌"
    }
}

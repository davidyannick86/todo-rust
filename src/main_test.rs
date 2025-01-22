use std::time::SystemTime;

#[cfg(test)]
#[test]
fn test_add_todo() {
    let mut todos = super::Todos(Vec::new());
    todos.add("Learn Rust".to_string());
    todos.add("Write a blog post".to_string());
    todos.add("Read a book".to_string());

    let expected = 3;
    let actual = todos.0.len();

    assert_eq!(expected, actual);
}

#[test]
fn test_validate_index() {
    let mut todos = super::Todos(Vec::new());
    todos.add("Learn Rust".to_string());
    todos.add("Write a blog post".to_string());
    todos.add("Read a book".to_string());

    let actual = todos.validate_index(0);

    assert!(actual.is_ok());
}

#[test]
fn delete_todo() {
    let mut todos = super::Todos(Vec::new());
    todos.add("Learn Rust".to_string());
    todos.add("Write a blog post".to_string());
    todos.add("Read a book".to_string());

    let actual = todos.delete_todo(1);

    assert!(actual.is_ok());
    assert!(todos.0.len() == 2);
}

#[test]
fn test_convert_time() {
    let datetime = SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(9741600);
    let expected = "23-04-1970 19:00:00".to_string();
    let actual = super::convert_time(datetime);

    assert_eq!(expected, actual);
}

use rusqlite::{params, Connection, Result};
use crate::task_controller::Task;

pub fn get_connection() -> Connection {
    let db_path = "tasks.db";
    Connection::open(&db_path).unwrap_or_else(|_| {
        panic!("Failed to open or create database at {}", db_path)
    })
}
pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT * FROM tasks WHERE status = 'active'")?;
    let task_iter = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            title: row.get(1)?,
            context: row.get(2)?,
            start_date: row.get(3)?,
            end_date: row.get(4)?,
            expected_end_date: row.get(5)?,
            tags: row.get::<_, String>(6)?.split(',').map(String::from).collect(),
            notes: row.get(7)?,
            status: row.get(8)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

pub fn create_task(conn: &Connection, task: &Task) -> Result<i64> {
    conn.execute(
        "INSERT INTO tasks (title, context, start_date, end_date, expected_end_date, tags, notes, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            task.title,
            task.context,
            task.start_date,
            task.end_date,
            task.expected_end_date,
            task.tags.join(","),
            task.notes,
            task.status,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn update_task(conn: &Connection, id: i64, task: &Task) -> Result<()> {
    conn.execute(
        "UPDATE tasks SET title = ?1, context = ?2, start_date = ?3, end_date = ?4, 
         expected_end_date = ?5, tags = ?6, notes = ?7, status = ?8 WHERE id = ?9",
        params![
            task.title,
            task.context,
            task.start_date,
            task.end_date,
            task.expected_end_date,
            task.tags.join(","),
            task.notes,
            task.status,
            id,
        ],
    )?;
    Ok(())
}

pub fn initialize_database() -> Result<(), rusqlite::Error> {
    let conn = get_connection();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            context TEXT,
            start_date TEXT,
            end_date TEXT,
            expected_end_date TEXT,
            tags TEXT,
            notes TEXT,
            status TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}
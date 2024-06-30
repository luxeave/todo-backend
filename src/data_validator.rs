use crate::task_controller::Task;

pub fn validate_task(task: &Task) -> Result<(), String> {
    println!("Validating task: {:?}", task);
    if task.title.is_empty() {
        return Err("Title is required".to_string());
    }
    if task.title.len() > 100 {
        return Err("Title must be 100 characters or less".to_string());
    }
    if let Some(context) = &task.context {
        if context.len() > 500 {
            return Err("Context must be 500 characters or less".to_string());
        }
    }
    if let Some(notes) = &task.notes {
        if notes.len() > 1000 {
            return Err("Notes must be 1000 characters or less".to_string());
        }
    }
    if task.tags.len() > 10 {
        return Err("Maximum of 10 tags allowed".to_string());
    }
    for tag in &task.tags {
        if tag.len() > 30 {
            return Err("Each tag must be 30 characters or less".to_string());
        }
    }
    
    // Validate dates
    if let (Some(start), Some(end)) = (&task.start_date, &task.end_date) {
        if end < start {
            return Err("End date must be after or equal to the start date".to_string());
        }
    }
    if let (Some(start), Some(expected)) = (&task.start_date, &task.expected_end_date) {
        if expected < start {
            return Err("Expected end date must be after or equal to the start date".to_string());
        }
    }

    Ok(())
}
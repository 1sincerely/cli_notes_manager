use crate::models::Note;


pub async fn show_notes(vec: &Vec<Note>){
    for note in vec {
        println!("{} | {} | {} | {} | {}", note.id, note.title, note.content, note.created_at, note.updated_at);
    }
}

pub async fn remove_handle(rows_affected: &u64) {
    if *rows_affected == 0 {
        println!("can't find note with that id!");
    } else {
        println!("{} rows affected!", rows_affected);
    }
}
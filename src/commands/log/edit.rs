use chrono::{DateTime, Local, TimeZone};
use colored::Colorize;

use crate::lib::{data::{logbook::Logbook, logentry::encode_date_time}, ensure_storage_location_exists, get_data_books_path};

pub fn exec_edit_log(
    id: &str,
    callsign: Option<&str>,
    frequency: Option<f32>,
    mode: Option<&str>,
    rst_sent: Option<&str>,
    rst_recv: Option<&str>,
    date: Option<&str>,
    time: Option<&str>,
    grid: Option<&str>,
    name: Option<&str>,
    notes: Option<&str>,
) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Split the ID into relevant parts
    let mut split_index = id.split(':').collect::<Vec<&str>>();
    let logbook = split_index.iter()
        .nth(0)
        .expect("ID must contain two parts separated by a colon");
    let index: usize = split_index.iter()
        .nth(1)
        .expect("ID must contain two parts separated by a colon")
        .parse()
        .expect("Second half of ID must be an integer");

    // Check if the book exists
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", logbook));
    if !std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook does not exist!".bright_red());
        return;
    }

    // Load the book
    let mut book: Logbook = autojson::structify(&book_path).expect("Could not deserialize logbook");

    // Try to fetch the log entry
    let entry = book.entries.iter_mut().nth(index);

    if entry.is_none() {
        println!("{}", "Entry does not exist!".bright_red());
        return;
    }
    let mut entry = entry.unwrap();

    // Handle new data
    if let Some(callsign) = callsign {
        entry.callsign = callsign.to_string();
    }
    if let Some(frequency) = frequency {
        entry.frequency_khz = frequency;
    }
    if let Some(mode) = mode {
        entry.mode = mode.to_string();
    }
    if let Some(rst_sent) = rst_sent {
        entry.sent_rst = Some(rst_sent.to_string());
    }
    if let Some(rst_recv) = rst_recv {
        entry.recv_rst = Some(rst_recv.to_string());
    }
    if let Some(grid) = grid {
        entry.grid = Some(grid.to_string());
    }
    if let Some(name) = name {
        entry.name = Some(name.to_string());
    }
    if let Some(notes) = notes {
        entry.notes = Some(notes.to_string());
    }

    // Handle date and time
    let new_date = match date {
        Some(date) => date.to_string(),
        None => entry.time.date().naive_local().to_string()
    };
    let new_time = match time {
        Some(time) => time.to_string(),
        None => {
            let local: DateTime<Local> = DateTime::from(entry.time);
            let split_time = local.time().to_string();
            let split_time = split_time.split(":").collect::<Vec<&str>>();
            format!("{}:{}", split_time.iter().nth(0).unwrap(), split_time.iter().nth(1).unwrap())
        }
    };
    
    // Set the time
    entry.time = encode_date_time(Some(new_date), Some(new_time), false);

    // Write the book
    autojson::jsonify(&book, &book_path).expect("Failed to serialize logbook");

    // Log the entry id
    println!("Updated log entry");
}

use chrono::{Date, NaiveDate, NaiveTime, Timelike, Utc};
use colored::Colorize;

use crate::lib::{
    data::{logbook::Logbook, logentry::LogEntry},
    ensure_storage_location_exists, get_data_books_path, get_data_default_book_path,
};

pub fn exec_new_book(
    name: &str,
    grid: Option<&str>,
    callsign: Option<&str>,
    description: Option<&str>,
    default: bool,
    import: Option<&str>,
) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // If the book already exists, error out
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", name));
    if std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook already exists!".bright_red());
        println!("Did you mean to use the \"edit\" command?");
        return;
    }

    // Create a logbook
    let mut book = Logbook {
        name: name.to_string(),
        grid: grid.map(Into::into),
        callsign: callsign.map(Into::into),
        description: description.map(Into::into),
        entries: Vec::new(),
    };

    // Handle import
    if let Some(import_path) = import {
        // Load the file
        let contents = std::fs::read_to_string(import_path).unwrap();

        // Parse to ADIF data
        let data = adif::parse_adif(&contents);

        println!("Processing your ADIF data...");

        // Fill in all records
        for record in data.body {
            if !record.is_empty() {
                // Fetch all needed data
                let entry = LogEntry::new_strings(
                    // Parse the Callsign
                    match record.get("CALL").expect("Missing ADIF field \"CALL\"") {
                        adif::AdifType::Str(val) => val.to_string(),
                        _ => panic!("Found a callsign that is not a string!"),
                    },
                    // Parse the frequency
                    match record.get("FREQ").expect("Missing ADIF field \"FREQ\"") {
                        adif::AdifType::Str(val) => {
                            lexical::parse::<f32, _>(val).expect("Frequency is not a number!")
                                * 1000.0
                        }
                        adif::AdifType::Number(val) => (val * 1000.0) as f32,
                        _ => panic!("Found a frequency that is not a string or number!"),
                    },
                    // Parse the mode of operation
                    match record.get("MODE").expect("Missing ADIF field \"MODE\"") {
                        adif::AdifType::Str(val) => val.to_string(),
                        _ => panic!("Found a mode that is not a string!"),
                    }, 
                    // Parse the sent RST
                    match record.get("RST_SENT") {
                        Some(s) => Some(match s {
                            adif::AdifType::Str(val) => val.to_string(),
                            _ => panic!("Found an RST that is not a string!"),
                        }),
                        None => None,
                    }, 
                    // Parse received RST
                    match record.get("RST_RCVD") {
                        Some(s) => Some(match s {
                            adif::AdifType::Str(val) => val.to_string(),
                            _ => panic!("Found a RST that is not a string!"),
                        }),
                        None => None,
                    },
                    // Parse the date
                    Some(
                        match record
                            .get("QSO_DATE")
                            .expect("Missing ADIF field \"QSO_DATE\"")
                        {
                            adif::AdifType::Str(val) => {
                                let date: Date<Utc> = Date::from_utc(
                                    NaiveDate::parse_from_str(val, "%Y%m%d").unwrap(),
                                    Utc,
                                );
                                date.to_string()
                            }
                            adif::AdifType::Date(val) => val.to_string(),
                            _ => panic!("Found a date that is not a string or date type!"),
                        },
                    ),
                    // Parse the time
                    Some(
                        match record
                            .get("TIME_ON")
                            .expect("Missing ADIF field \"TIME_ON\"")
                        {
                            adif::AdifType::Str(val) => {
                                let time = NaiveTime::parse_from_str(val, "%H%M%S").unwrap();
                                format!("{}:{}", time.hour(), time.minute())
                            }
                            adif::AdifType::Time(val) => val.to_string(),
                            _ => panic!("Found a time that is not a string or time type!"),
                        },
                    ),
                    // Parse the gridsquare
                    match record.get("GRIDSQUARE") {
                        Some(s) => Some(match s {
                            adif::AdifType::Str(val) => val.to_string(),
                            _ => panic!("Found a gridsquare that is not a string!"),
                        }),
                        None => None,
                    },
                    // Parse the name
                    match record.get("NAME") {
                        Some(s) => Some(match s {
                            adif::AdifType::Str(val) => val.to_string(),
                            _ => panic!("Found a name that is not a string!"),
                        }),
                        None => None,
                    },
                    // Dont import notes
                    None,
                )
                .expect("Failed to parse imported entry");

                // Add the entry
                book.entries.push(entry);
            }
        }
    }

    // Save the logbook
    autojson::jsonify(&book, &book_path).expect("Failed to serialize logbook");

    // Handle saving this as the default
    if default {
        std::fs::write(get_data_default_book_path(), name).expect("Failed to set book as default");
        println!("Set book as default");
    }

    // Log success
    println!("New logbook created");
}

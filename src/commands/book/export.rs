use adif::{AdifFile, AdifHeader, AdifType};
use chrono::Duration;
use clap::crate_version;
use colored::Colorize;
use indexmap::indexmap;

use crate::lib::{data::logbook::Logbook, ensure_storage_location_exists, get_data_books_path};

pub fn exec_export_book(name: &str, outfile: &str) {
    // Ensure we have our directories
    ensure_storage_location_exists();

    // Check if the book exists
    let mut book_path = get_data_books_path();
    book_path.push(format!("{}.json", name));
    if !std::fs::metadata(&book_path).is_ok() {
        println!("{}", "Logbook does not exist!".bright_red());
        return;
    }

    // Load the book
    let book: Logbook = autojson::structify(&book_path).expect("Could not deserialize logbook");

    // Build the file header
    let header: AdifHeader = indexmap! {
        "ADIF_VERS" => AdifType::Str("3.1.0".to_string()),
        "PROGRAMID" => AdifType::Str("CL".to_string()),
        "PROGRAMVERSION" => AdifType::Str(crate_version!().to_string()),
    }
    .into();

    // Build the file
    let file = AdifFile {
        header,
        body: book
            .entries
            .iter()
            .map(|entry| {
                return indexmap! {
                    "CALL" => AdifType::Str(entry.callsign.clone()),
                    "BAND" => AdifType::Str(entry.band.clone()),
                    "MODE" => AdifType::Str(entry.mode.clone()),
                    "FREQ" => AdifType::Number(((entry.frequency_khz as f64 * 1000.0).round() / 1000.0) / 1000.0 ),
                    "QSO_DATE" => AdifType::Date(entry.time.date()),
                    "TIME_ON" => AdifType::Time(entry.time.time()),
                    "TIME_OFF" => AdifType::Time(entry.time.time() + Duration::minutes(5)),
                    "RST_RECVD" => AdifType::Str(entry.recv_rst.clone().unwrap_or("".to_string())),
                    "RST_SENT" => AdifType::Str(entry.sent_rst.clone().unwrap_or("".to_string())),
                    "GRIDSQUARE" => AdifType::Str(entry.grid.clone().unwrap_or("".to_string())),
                    "NAME" => AdifType::Str(entry.name.clone().unwrap_or("".to_string())),
                    "TX_PWR" => AdifType::Number(entry.tx_pwr.unwrap_or(0.0) as f64),
                }.into();
            })
            .collect(),
    };

    // Write out the file
    std::fs::write(outfile, file.serialize().unwrap()).expect("Could not save exported file");
}

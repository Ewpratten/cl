use std::path::PathBuf;

pub mod data;


pub fn get_data_root_path() -> PathBuf {
    let mut out = dirs::home_dir().expect("Could not find user home directory!");
    out.push(".cl.data");
    out
}

pub fn get_data_books_path() -> PathBuf {
    let mut out = get_data_root_path();
    out.push("books");
    out
}

pub fn get_data_default_book_path() -> PathBuf {
    let mut out = get_data_root_path();
    out.push("default-book");
    out
}

pub fn ensure_storage_location_exists() {
    if !std::fs::metadata(get_data_books_path()).is_ok() {
        std::fs::create_dir_all(get_data_books_path())
            .expect("Failed to create logbook directory");
    }
}

use std::process::Command;

pub fn exec_publish_book(name: &str, outfile: &str) {
    // Run command
    Command::new("tqsl")
        .arg("-x")
        .arg("-u")
        .arg(outfile)
        .output()
        .expect("Failed to execute TQSL. Is it installed and on the $PATH?");
}

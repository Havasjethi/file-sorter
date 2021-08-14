mod file_sorter;

use crate::file_sorter::FileSorter;
use clap::{App, Arg};

fn main() {
    let matches = App::new("File sorter")
        .version("0.1.0")
        .arg(
            Arg::new("Folder")
                .about("FolderPath")
                .default_value(".")
                .index(1),
        )
        .get_matches();

    if let Some(folder) = matches.value_of("Folder") {
        FileSorter::create(folder.to_owned());
    }
}

mod file_watcher;

use crate::file_watcher::FileWatcher;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Gec")
        .version("0.0.1")
        .arg(
            Arg::new("Folder")
                .about("FolderPath")
                .default_value(".")
                .index(1),
        )
        .get_matches();

    if let Some(folder) = matches.value_of("Folder") {
        FileWatcher::create(folder.to_owned());
    }
}

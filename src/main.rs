mod file_watcher;

use crate::file_watcher::FileWatcher;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Gec")
        .version("0.0.1")
        .arg(
            Arg::new("Folder")
                .about("FolderPath")
                .required(true)
                .index(1),
        )
        .get_matches();

    if let Some(folder) = matches.value_of("Folder") {
        let watcher = FileWatcher::create(folder.to_owned());
        watcher.watch_folder();
    }
}

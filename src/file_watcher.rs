use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};

pub struct FileWatcher {
    folder: PathBuf,
}

impl FileWatcher {
    pub fn create(string: String) -> FileWatcher { FileWatcher { folder: PathBuf::from(string) } }

    pub fn watch_folder(&self) {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(
            tx,
            Duration::from_secs(1),
        ).expect("asd");

        watcher.watch(&self.folder, RecursiveMode::NonRecursive);
        std::fs::read_dir(&self.folder)
            .expect("Folder doesn't exists!")
            .for_each(|e| {
                if let Ok(entity) = e {
                    self.move_file(entity.path())
                }
            });

        loop {
            match rx.recv() {
                Ok(event) => self.handle_event(event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }

    fn handle_event(&self, event: DebouncedEvent) {
        println!("{:?}", event);

        match event {
            DebouncedEvent::Create(file) => self.move_file(file),
            _ => (),
        }
    }

    fn on_startup(&self) {
        // Read folder items

        // Move each item
    }

    fn move_file(&self, source_path: PathBuf) {
        if !source_path.is_file() {
            return;
        }

        println!("Moving file    :: {:?}", &source_path);

        if let Some(extension) = source_path.extension() {
            let mut dest_folder = self.folder.clone();

            dest_folder.push(extension);

            if !dest_folder.exists() {
                std::fs::create_dir(&dest_folder);
            }

            let destination_path: PathBuf = {
                let mut mut_last_name = source_path.file_stem().expect("Nagy baj van")
                    .to_str()
                    .and_then(|x| Some(x.to_owned())).expect("Baj van");

                let extension = source_path.extension()
                    .and_then(|x| x.to_str())
                    .expect("Nagy baj van");

                let create_filename = |x: &String, y: &str| {
                    let mut n = x.clone();
                    n.push('.');
                    n.push_str(y);
                    return dest_folder.join(n);
                };

                let mut current_iter = 0;
                let max_iter = 15;

                loop {
                    current_iter = current_iter + 1;

                    let file_name = create_filename(&mut_last_name, extension);

                    if !file_name.exists() {
                        break file_name;
                    }

                    mut_last_name.push('_');

                    if current_iter > max_iter {
                        panic!("Max iteration reached");
                    }
                }
            };

            println!("Moving file to :: {:?}", &destination_path);

            std::fs::copy(source_path, destination_path);
        }
    }

    fn create_folder(&self, path: PathBuf) {}
}

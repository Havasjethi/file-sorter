use std::ffi::OsStr;
use std::fs::{create_dir, read_dir, rename};
use std::path::{Path, PathBuf};

pub struct FileWatcher {
    folder: PathBuf,
}

impl FileWatcher {
    pub fn create(string: String) -> FileWatcher {
        let instance = FileWatcher {
            folder: PathBuf::from(string),
        };

        instance.on_startup();

        instance
    }


    fn on_startup(&self) {
        read_dir(&self.folder)
            .expect("Folder doesn't exists!")
            .for_each(|e| {
                if let Ok(entity) = e {
                    self.move_file(entity.path())
                }
            });
    }

    fn move_file(&self, source_path: PathBuf) {
        if !source_path.is_file() {
            return;
        }

        println!("Moving file    :: {:?}", &source_path);


        if let Some(extension) = source_path.extension() {
            let dest_folder = self.folder.join(extension);

            if !dest_folder.exists() {
                create_dir(&dest_folder)
                    .expect("Folder destination cannot be created");
            }

            if let Ok(destination_path) = self.create_destination(&source_path, &dest_folder) {
                println!("Moving file to :: {:?}", &destination_path);

                rename(source_path, &destination_path)
                    .unwrap_or_else(|_| panic!("Cannot move file to {:?}", &destination_path))
                        // expect(&format!("Cannot move file to {:?}", &destination_path));
            };
        }
    }

    fn create_destination(&self, source_path: &Path, dest_folder: &Path) -> Result<PathBuf, String> {
        let mut mut_last_name = source_path
            .file_stem()
            .map(|x| x.to_os_string())
            .expect("Nagy baj van");

        let extension = source_path
            .extension()
            .expect("Nagy baj van");

        let create_filename = |x: &OsStr, y: &OsStr| {
            let mut n = x.to_os_string();
            n.push(OsStr::new("."));
            n.push(y);

            dest_folder.join(n)
        };

        let mut current_iter = 0;
        let max_iter = 15;

        loop {
            current_iter += 1;

            let file_name = create_filename(&mut_last_name, extension);

            if !file_name.exists() {
                return Ok(file_name);
            }

            mut_last_name.push(OsStr::new("_"));

            if current_iter > max_iter {
                return Err("Max iteration reached".to_owned());
            }
        }
    }
}

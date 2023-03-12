use std::{
    env, fs,
    path::{Path, PathBuf},
};

const COPY_DIR: [&'static str; 6] = ["css", "identity", "image", "contacts", "templates", "bills"];

/// A helper function for recursively copying a directory.
fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir(&to).unwrap();
            }

            copy_dir(&path, to);
        } else { /* Skip other content */
        }
    }
}

fn main() {
    init_folders();
    let out = env::var("PROFILE").unwrap();
    for dir in COPY_DIR {
        let out = PathBuf::from(format!("target/{}/{}", out, dir));

        // If it is already in the output directory, delete it and start over
        if out.exists() {
            fs::remove_dir_all(&out).unwrap();
        }

        // Create the out directory
        fs::create_dir(&out).unwrap();

        // Copy the directory
        copy_dir(dir, &out);
    }
}

fn init_folders() {
    if !Path::new("contacts").exists() {
        fs::create_dir("contacts").expect("Can't create folder contacts.");
    }
    if !Path::new("identity").exists() {
        fs::create_dir("identity").expect("Can't create folder identity.");
    }
    if !Path::new("bills").exists() {
        fs::create_dir("bills").expect("Can't create folder identity.");
    }
    if !Path::new("css").exists() {
        fs::create_dir("css").expect("Can't create folder identity.");
    }
    if !Path::new("image").exists() {
        fs::create_dir("image").expect("Can't create folder identity.");
    }
    if !Path::new("templates").exists() {
        fs::create_dir("templates").expect("Can't create folder identity.");
    }
}

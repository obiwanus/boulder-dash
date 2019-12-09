use std::env;
use std::fs::{self, DirBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_dir = {
        if let Some(target_dir) = env::var("CARGO_TARGET_DIR").ok() {
            PathBuf::from(target_dir)
        } else {
            project_dir.join("target")
        }
    };
    let exe_path = target_dir.join(env::var("PROFILE").unwrap());

    copy(project_dir.join("assets"), exe_path.join("assets"));
}

fn copy(from: PathBuf, to: PathBuf) {
    for entry in WalkDir::new(&from) {
        let source = entry.unwrap();
        let rel_path = source.path().strip_prefix(&from).unwrap();
        let target_path = to.join(rel_path);
        if source.file_type().is_dir() {
            DirBuilder::new()
                .recursive(true)
                .create(&target_path)
                .expect("Couldn't create dir");
            println!("Created dir: {:?}", target_path);
        } else {
            fs::copy(source.path(), &target_path).expect("Failed to copy file");
            println!("Copied {:?} to {:?}", source.path(), target_path);
        }
    }
}

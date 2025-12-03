use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_env() {
    dotenvy::dotenv().expect("Failed to read .env file");
}

pub fn load_file<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let workspace_root = std::env::var("WORKSPACE_ROOT").expect("WORKSPACE_ROOT not set");
    let workspace_root_path = Path::new(&workspace_root);
    let file_path = workspace_root_path.join(filename.as_ref());

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    reader.lines().collect()
}

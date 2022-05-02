use std::fs::File;

pub fn get_file(path: &str) -> File {
    let file = File::open(path);

    let x = match file {
        Ok(x) => x,
        Err(e) => {
            panic!("Error opening file: {}", e)
        }
    };

    x
}

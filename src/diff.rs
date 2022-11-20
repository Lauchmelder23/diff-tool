use std::{fs::File, io::{BufReader, Read}};

use crate::grid::Grid;

macro_rules! safe_unwrap {
    ($expression: expr) => {
        match $expression {
            Ok(res) => res,
            Err(err) => return Err(err.to_string())
        }
    };
}

#[derive(Default, Clone)]
enum Arrow {
    #[default] LEFT,
    UP,
    DIAGONAL
}

fn open_and_read(file: String) -> Result<String, std::io::Error> {
    let fp = File::open(file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

pub struct Diff {
    first: String,
    second: String,
}

impl Diff {
    pub fn new(first: String, second: String) -> Result<Diff, String> {
        let first_string = safe_unwrap!(open_and_read(first));
        let second_string = safe_unwrap!(open_and_read(second));

        let mut diff = Diff {
            first: first_string,
            second: second_string
        };



        Ok(diff)
    }

    fn create_lcs(&self) -> Grid<Arrow> {
        Grid::new(self.first.len() as u32, self.second.len() as u32)
    }
}
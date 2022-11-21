use std::{fs::File, io::{BufReader, Read}, fmt::Display};
use crate::grid::Grid;
use colored::*;

macro_rules! safe_unwrap {
    ($expression: expr) => {
        match $expression {
            Ok(res) => res,
            Err(err) => return Err(err.to_string())
        }
    };
}

#[derive(Clone)]
enum Arrow {
    NONE,
    LEFT,
    UP,
    DIAGONAL
}

impl Display for Arrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arrow::NONE => write!(f, "  "),
            Arrow::LEFT => write!(f, "<-"),
            Arrow::UP => write!(f, "^^"),
            Arrow::DIAGONAL => write!(f, "<^")
        }
    }
}

#[derive(Debug)]
enum Action {
    INSERT(u32, char),
    DELETE(u32),
    REPLACE(usize, char)
}

fn open_and_read(file: String) -> Result<String, std::io::Error> {
    let fp = File::open(file)?;
    let mut buf_reader = BufReader::new(fp);
    let mut content = String::new();

    buf_reader.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Clone)]
struct Cell(usize, Arrow);

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Diff {
    first: String,
    second: String,

    actions: Vec<Action>
}

impl Diff {
    pub fn new(first: String, second: String) -> Result<Diff, String> {
        let first_string = safe_unwrap!(open_and_read(first));
        let second_string = safe_unwrap!(open_and_read(second));

        let mut diff = Diff {
            first: first_string,
            second: second_string,

            actions: vec![]
        };

        let grid = diff.create_lcs();

        let mut pos = (diff.first.len() as u32, diff.second.len() as u32);
        while pos != (0, 0) {
            pos = match grid[pos].1 {
                Arrow::DIAGONAL => (pos.0 - 1, pos.1 - 1),
                Arrow::UP => {
                    diff.actions.push(Action::INSERT(pos.0, diff.second.chars().nth(pos.1 as usize - 1).unwrap()));
                    (pos.0, pos.1 - 1)
                },
                Arrow::LEFT => {
                    diff.actions.push(Action::DELETE(pos.0));
                    (pos.0 - 1, pos.1)
                },
                Arrow::NONE => {
                    break;
                }
            }
        }

        Ok(diff)
    }

    pub fn get_input(&self) -> (&String, &String) {
        (&self.first, &self.second)
    }

    pub fn step_by_step(&self) -> Vec<String> {
        let mut progression = vec![self.first.to_owned()];
        
        for action in &self.actions {
            let mut text = progression.last().unwrap().to_owned();

            match action {
                Action::INSERT(pos, chr) => { text.insert(*pos as usize, *chr); },
                Action::DELETE(pos) => { text.remove(*pos as usize - 1); },
                Action::REPLACE(_, _) => unimplemented!()
            };

            progression.push(text);
        }

        progression
    }

    fn create_lcs(&self) -> Grid<Cell> {
        let mut grid = Grid::new(
            self.first.len() as u32 + 1, 
            self.second.len() as u32 + 1, 
            Cell(0, Arrow::NONE)
        );

        self.first.chars().enumerate().for_each(|(i, _)| grid[(i as u32 + 1, 0u32)] = Cell(0, Arrow::LEFT));
        self.second.chars().enumerate().for_each(|(i, _)| grid[(0u32, i as u32 + 1)] = Cell(0, Arrow::UP));

        grid[(0, 0)] = Cell(0, Arrow::NONE);

        for (i, x) in self.first.chars().enumerate().map(|(i, c)| (i as u32 + 1, c)) {
            for (j, y) in self.second.chars().enumerate().map(|(i, c)| (i as u32 + 1, c)) {

                if x == y {
                    grid[(i, j)] = Cell(grid[(i - 1, j - 1)].0 + 1, Arrow::DIAGONAL);
                } else {
                    let left = grid[(i - 1, j)].0;
                    let up = grid[(i, j - 1)].0;

                    grid[(i, j)] = match up >= left {
                        true => Cell(up, Arrow::UP),
                        false => Cell(left, Arrow::LEFT)
                    };
                }
            }
        }

        grid
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = self.first.chars().map(|c| c.to_string().normal()).collect::<Vec<ColoredString>>();

        for action in &self.actions {
            match action {
                Action::INSERT(pos, chr) => { chars.insert(*pos as usize, chr.to_string().green()) },
                Action::DELETE(pos) => { chars[*pos as usize - 1] = chars[*pos as usize - 1].to_owned().red().strikethrough() },
                Action::REPLACE(_, _) => unimplemented!()
            };
        }

        for chr in chars {
            write!(f, "{chr}")?;
        }

        Ok(())
    }
}
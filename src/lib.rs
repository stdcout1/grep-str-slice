use std::{fs, fmt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrepError {
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("not enough arguments")]
    NotEnoughArguments,
}

pub struct Grep<'a> {
    needle: &'a str,
    haystack: &'a str,
    previous_line: usize,
}
#[derive(Debug)]
pub struct GrepIteration<'a> {
    line: usize,
    column: usize,
    line_content: &'a str,
}

type GrepResult<'a> = Result<Grep<'a>, GrepError>;

impl fmt::Display for GrepIteration<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}:{}", self.line_content, self.line, self.column)
    }    
}

impl<'a, 'b> Grep<'a> {
    pub fn new_str_search(needle: &'a str, haystack: &'a str) -> GrepResult<'a> {
        GrepResult::Ok(
            Self { needle, haystack: &haystack, previous_line: 0 }
        )
    }
    pub fn in_file_search(needle: &'a str, file_path: &'b str ) -> GrepResult<'a> {
        let haystack = Box::leak::<'a>(fs::read_to_string(file_path)?.into_boxed_str()); // we want this to live for the rest of time
        Self::new_str_search(needle, haystack)
    }
    pub fn from_args(args: &'a Vec<String>) -> GrepResult<'a> {
        let needle = &args.get(1).ok_or(GrepError::NotEnoughArguments)?;
        let file_path = &args.get(2).ok_or(GrepError::NotEnoughArguments)?;
        Self::in_file_search(needle, file_path)
    }
}

impl<'a> Iterator for Grep <'a> {
    type Item = GrepIteration<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.haystack.lines().find(|line| line.contains(self.needle))?;
        let (before, after) = self.haystack.split_once(line)?;
        self.haystack = after;
        self.previous_line += before.lines().count();
        Some(
            GrepIteration {
                line: self.previous_line,
                column: line.find(self.needle)?,
                line_content: line,
            }
        )
    }
}

use crate::game::Game;
use std::cell::RefCell;
use std::io;

impl LineReader for STDInReader {
    fn read_line(&self, input: &mut String) {
        io::stdin().read_line(input).unwrap();
    }
}

#[derive(Clone)]
pub struct MockLineReader {
    pub(crate) index: RefCell<usize>,
    pub(crate) lines: Vec<String>,
}

impl Default for MockLineReader {
    fn default() -> Self {
        MockLineReader {
            index: RefCell::new(0),
            lines: vec!["2 3".to_string(), "1 2".to_string()],
        }
    }
}

impl LineReader for MockLineReader {
    fn read_line(&self, input: &mut String) {
        let mut index = self.index.borrow_mut();
        let line = self.lines.get(*index);
        *index = *index + 1;
        *input = line.unwrap().clone();
    }
}

pub trait LineReader: Default + Clone {
    fn read_line(&self, input: &mut String);
}

#[derive(Debug, Default, Clone)]
pub struct STDInReader {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::line_reader::{LineReader, MockLineReader};

    #[test]
    fn test_mock_line_reader() {
        let mock = MockLineReader::default();
        let mut input = String::new();
        mock.read_line(&mut input);
        assert!(input.eq("2 3"));
        mock.read_line(&mut input);
        assert!(input.eq("1 2"));
    }
}

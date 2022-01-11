use std::fmt;
use std::ops::Index;

pub mod game;
pub mod solver;
pub mod words;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Letter(u8);

impl Letter {
    pub fn from_byte(byte: u8) -> Self {
        if !(b'a'..=b'z').contains(&byte) {
            panic!("letter {} out of bounds", byte)
        }
        Self(byte - 97)
    }
    pub fn from_char(c: char) -> Self {
        if !('a'..='z').contains(&c) {
            panic!("letter {} out of bounds", c)
        }
        Self::from_byte(c as u8)
    }
    pub fn as_index(&self) -> usize {
        (self.0).into()
    }
    pub fn as_byte(&self) -> u8 {
        self.0 + 97
    }
    pub fn as_char(&self) -> char {
        self.as_byte() as char
    }
}

impl From<Letter> for usize {
    fn from(letter: Letter) -> Self {
        letter.as_index()
    }
}

impl From<Letter> for u8 {
    fn from(letter: Letter) -> Self {
        letter.as_byte()
    }
}

impl From<Letter> for char {
    fn from(letter: Letter) -> Self {
        letter.as_char()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    pub vec: Vec<Letter>,
}

impl Word {
    pub fn new() -> Self {
        Self { vec: vec![] }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
}

impl Default for Word {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&str> for Word {
    fn from(s: &str) -> Self {
        Self {
            vec: s.chars().map(Letter::from_char).collect(),
        }
    }
}

impl From<&Word> for String {
    fn from(word: &Word) -> Self {
        word.vec.iter().map(|l| l.as_char()).collect()
    }
}

impl fmt::Display for Word {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.into();
        fmt.write_str(&s)
    }
}

impl Index<usize> for Word {
    type Output = Letter;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.vec[idx]
    }
}

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use lazy_static::lazy_static;

const SCRABBLE_WORDS: &str = include_str!("data/scrabble.txt");
const DICTIONARY_WORDS: &str = include_str!("data/dictionary.txt");
const WORDLE_WORDS: &str = include_str!("data/wordle.txt");
const WORDS_FREQUENCY: &str = include_str!("data/frequency.txt");

lazy_static! {
    static ref WORD_FREQUENCIES: HashMap<String, u64> = {
        let mut map = HashMap::new();
        for wordfreq in WORDS_FREQUENCY.split('\n') {
            let parts: Vec<&str> = wordfreq.split('\t').collect();
            if parts.len() == 2 {
                let word = parts[0];
                let freq: u64 = parts[1].parse().unwrap();
                map.insert(word.to_string(), freq);
            } else {
                panic!("bad frequency word list");
            }
        }
        map
    };
}

#[derive(Debug, Clone)]
pub enum WordSource {
    Wordle,
    Scrabble { letter_count: usize, top_n: usize },
    Dictionary { letter_count: usize, top_n: usize },
}

impl WordSource {
    pub fn letter_count(&self) -> usize {
        match self {
            Self::Wordle => 5,
            Self::Scrabble { letter_count, .. } => *letter_count,
            Self::Dictionary { letter_count, .. } => *letter_count,
        }
    }
}

impl FromStr for WordSource {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        if s == "wordle" {
            return Ok(WordSource::Wordle);
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err("invalid word source".to_string());
        }
        let source = parts[0];
        let letter_count: usize = match parts[1].parse() {
            Ok(n) => n,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        let top_n: usize = match parts[2].parse() {
            Ok(n) => n,
            Err(e) => {
                return Err(e.to_string());
            }
        };
        if source == "scrabble" {
            return Ok(WordSource::Scrabble {
                letter_count,
                top_n,
            });
        }
        if source == "dictionary" {
            return Ok(WordSource::Dictionary {
                letter_count,
                top_n,
            });
        }
        Err("invalid word source".to_string())
    }
}

pub struct WordList {
    pub source: WordSource,
    pub allowed_solutions: HashSet<String>,
    pub words: HashSet<String>,
}

impl WordList {
    fn new<T: AsRef<str>>(words: &[T], allow_top_n_solutions: usize, source: WordSource) -> Self {
        let mut words: Vec<String> = words.iter().map(|v| v.as_ref().to_string()).collect();
        sort_by_frequency(&mut words);
        Self {
            source,
            words: words.iter().map(|v| v.to_string()).collect(),
            allowed_solutions: words[..allow_top_n_solutions]
                .iter()
                .map(|v| v.to_string())
                .collect(),
        }
    }
    pub fn from_source(source: &WordSource) -> Self {
        match source {
            WordSource::Wordle => Self::new(&wordle_words(), 2315, source.clone()),
            WordSource::Scrabble {
                letter_count,
                top_n,
            } => Self::new(&scrabble_words(*letter_count), *top_n, source.clone()),
            WordSource::Dictionary {
                letter_count,
                top_n,
            } => Self::new(&dictionary_words(*letter_count), *top_n, source.clone()),
        }
    }
    pub fn is_valid_guess(&self, word: &str) -> bool {
        self.words.contains(word)
    }
    pub fn is_valid_solution(&self, word: &str) -> bool {
        self.allowed_solutions.contains(word)
    }
}

fn is_valid_word(word: &str, letter_count: usize) -> bool {
    if word.len() != letter_count {
        return false;
    }
    for c in word.chars() {
        if !('a'..='z').contains(&c) {
            return false;
        }
    }
    true
}

fn get_words(wordlist: &str, letter_count: usize) -> Vec<String> {
    wordlist
        .split('\n')
        .filter(|w| is_valid_word(w, letter_count))
        .map(|w| w.to_string())
        .collect()
}

fn scrabble_words(letter_count: usize) -> Vec<String> {
    get_words(SCRABBLE_WORDS, letter_count)
}

fn dictionary_words(letter_count: usize) -> Vec<String> {
    get_words(DICTIONARY_WORDS, letter_count)
}

fn wordle_words() -> Vec<String> {
    get_words(WORDLE_WORDS, 5)
}

fn word_frequency(word: &str) -> u64 {
    *WORD_FREQUENCIES.get(word).unwrap_or(&0)
}

fn sort_by_frequency(words: &mut Vec<String>) {
    words.sort_by_key(|k| word_frequency(k));
    words.reverse()
}

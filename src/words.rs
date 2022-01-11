use std::collections::{HashMap, HashSet};

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

pub struct WordList {
    pub allowed_solutions: HashSet<String>,
    pub words: HashSet<String>,
}

impl WordList {
    fn new<T: AsRef<str>>(words: &[T], allow_top_n_solutions: usize) -> Self {
        let mut words: Vec<String> = words.iter().map(|v| v.as_ref().to_string()).collect();
        sort_by_frequency(&mut words);
        Self {
            words: words.iter().map(|v| v.to_string()).collect(),
            allowed_solutions: words[..allow_top_n_solutions]
                .iter()
                .map(|v| v.to_string())
                .collect(),
        }
    }
    pub fn from_source(source: &WordSource) -> Self {
        match source {
            WordSource::Wordle => Self::new(&wordle_words(), 2315),
            WordSource::Scrabble {
                letter_count,
                top_n,
            } => Self::new(&scrabble_words(*letter_count), *top_n),
            WordSource::Dictionary {
                letter_count,
                top_n,
            } => Self::new(&dictionary_words(*letter_count), *top_n),
        }
    }
    pub fn is_valid_guess(&self, word: &str) -> bool {
        self.words.contains(word)
    }
    pub fn is_valid_solution(&self, word: &str) -> bool {
        self.allowed_solutions.contains(word)
    }
}

fn get_words(wordlist: &str, letter_count: usize) -> Vec<String> {
    let mut words = vec![];
    for word in wordlist.split('\n') {
        let word = word.trim();
        if word.len() == letter_count {
            words.push(word.to_string());
        }
    }
    words
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

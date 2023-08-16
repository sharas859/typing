use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rand::seq::SliceRandom;
use rand::Rng;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct WordIndex<'a> {
    // todo: change hashset to vec and only use set for reading in words
    unigrams: HashMap<String, HashSet<&'a str>>,
    bigrams: HashMap<String, HashSet<&'a str>>,
}

trait GetRandom {
    fn get_random(&self) -> Option<&str>;
}
trait GetRandomFromKey {
    fn get_random_from_key(&self, key: &str) -> Option<&str>;
}

impl GetRandom for HashMap<String, HashSet<&str>> {
    fn get_random(&self) -> Option<&str> {
        let mut rng = rand::thread_rng();
        let keys: Vec<&String> = self.keys().collect();

        if let Some(&key) = keys.choose(&mut rng) {
            let word = self.get_random_from_key(key);
            return word;
        }
        None
    }
}

impl GetRandomFromKey for HashMap<String, HashSet<&str>> {
    fn get_random_from_key(&self, key: &str) -> Option<&str> {
        if let Some(words) = self.get(key) {
            let word = words.get_random();
            return word;
        }
        None
    }
}

impl GetRandom for HashSet<&str> {
    fn get_random(&self) -> Option<&str> {
        let mut rng = rand::thread_rng();
        let words_vec: Vec<&str> = self.iter().cloned().collect();
        if let Some(word) = words_vec.choose(&mut rng) {
            return Some(word);
        }
        None
    }
}

impl<'a> WordIndex<'a> {
    pub fn new() -> WordIndex<'a> {
        WordIndex {
            unigrams: HashMap::new(),
            bigrams: HashMap::new(),
        }
    }

    fn add_word(&mut self, word: &'a str) {
        for c in word.chars() {
            let c = c.to_string();
            let unigrams = self.unigrams.entry(c).or_insert(HashSet::new());
            unigrams.insert(word);
        }

        for bigram in word.chars().collect::<Vec<char>>().windows(2) {
            let bigram = bigram.iter().collect::<String>();
            let bigrams = self.bigrams.entry(bigram).or_insert(HashSet::new());
            bigrams.insert(word);
        }
    }

    pub fn read_words(&mut self, words: &'a str) {
        //split on newline
        for word in words.split_whitespace() {
            self.add_word(word);
        }
    }

    fn get_unigrams(&self, unigram: &str) -> Option<&HashSet<&str>> {
        self.unigrams.get(unigram)
    }

    fn get_bigrams(&self, bigram: &str) -> Option<&HashSet<&str>> {
        self.bigrams.get(bigram)
    }

    fn get_random_unigram_word(&self) -> Option<&str> {
        self.unigrams.get_random()
    }

    fn get_random_bigramm_word(&self) -> Option<&str> {
        self.bigrams.get_random()
    }

    fn get_word_from_bigramm(&self, bigram: &str) -> Option<&str> {
        self.bigrams.get_random_from_key(bigram)
    }

    fn get_word_from_unigram(&self, unigram: &str) -> Option<&str> {
        self.unigrams.get_random_from_key(unigram)
    }

    fn get_random_word(&self) -> Option<&str> {
        let mut rng = rand::thread_rng();
        //this is kinda unnecessary, since they contain the same words, but who knows
        let word = [&self.unigrams, &self.bigrams]
            .choose(&mut rng)
            .unwrap()
            .get_random();
        word
    }

    pub fn generate_random_lesson(&self, length: usize) -> Vec<&str> {
        let mut lesson = Vec::new();
        let mut lesson_len = 0;
        while lesson_len < length {
            let word = self.get_random_word();
            if let Some(word) = word {
                lesson_len += word.len();
                lesson.push(word);
            }
        }
        lesson
    }

    pub fn generate_lesson_vec_from_n_grams(
        &self,
        length: usize,
        n_grams: &Vec<String>,
    ) -> Vec<&str> {
        if n_grams.is_empty() {
            return self.generate_random_lesson(length);
        }

        //check if all n_gram entries are empty
        let mut all_empty = true;
        for n_gram in n_grams {
            if (n_gram.len() == 1 && self.get_unigrams(n_gram).is_some())
                || (n_gram.len() == 2 && self.get_bigrams(n_gram).is_some())
            {
                all_empty = false;
                break;
            }
        }
        if all_empty {
            return self.generate_random_lesson(length);
        }

        let mut lesson = Vec::new();
        let mut lesson_len = 0;
        let mut rng = rand::thread_rng();

        while lesson_len < length {
            let n_gram = n_grams.choose(&mut rng).unwrap();
            if n_gram.len() == 1 {
                let word = self.get_word_from_unigram(n_gram);
                if let Some(word) = word {
                    lesson_len += word.len();
                    lesson.push(word);
                }
            } else if n_gram.len() == 2 {
                let word = self.get_word_from_bigramm(n_gram);
                if let Some(word) = word {
                    lesson_len += word.len();
                    lesson.push(word);
                }
            }
        }

        lesson
    }

    fn generate_lesson_string(&self, lesson: Vec<&str>) -> String {
        lesson.join(" ")
    }

    pub fn generate_random_lesson_string(&self, length: usize) -> String {
        let lesson = self.generate_random_lesson(length);
        self.generate_lesson_string(lesson)
    }

    pub fn generate_lesson_string_from_ngrams_with_special_chars(
        &self,
        length: usize,
        n_grams: &Vec<String>,
        special_chars: &Vec<String>,
    ) -> String {
        let mut rng = rand::thread_rng();
        let lesson = self.generate_lesson_vec_from_n_grams(length, n_grams);
        lesson
            .iter()
            .map(|&s| {
                if let Some(c) = special_chars.choose(&mut rng) {
                    //should never be more than 1 char
                    let c = c.chars().next().unwrap();
                    // insert at random position
                    let pos = rng.gen_range(0..s.len());
                    let mut str = s.to_string();
                    str.insert(pos, c);
                    str
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn generate_lesson_from_n_grams(&self, length: usize, n_grams: &Vec<String>) -> String {
        let lesson = self.generate_lesson_vec_from_n_grams(length, n_grams);
        self.generate_lesson_string(lesson)
    }
}

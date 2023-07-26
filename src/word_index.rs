use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::Arc;

use leptos::html::S;
use rand::seq::SliceRandom;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct WordIndex {
    unigrams: HashMap<String, HashSet<Arc<String>>>,
    bigrams: HashMap<String, HashSet<Arc<String>>>,
}

trait GetRandom {
    fn get_random(&self) -> Option<Arc<String>>;
}
trait GetRandomFromKey {
    fn get_random_from_key(&self, key: &str) -> Option<Arc<String>>;
}

impl GetRandom for HashMap<String, HashSet<Arc<String>>> {
    fn get_random(&self) -> Option<Arc<String>> {
        let mut rng = rand::thread_rng();
        let keys: Vec<&String> = self.keys().collect();

        if let Some(&key) = keys.choose(&mut rng) {
            let word = self.get_random_from_key(key);
            return word;
        }
        None
    }
}

impl GetRandomFromKey for HashMap<String, HashSet<Arc<String>>> {
    fn get_random_from_key(&self, key: &str) -> Option<Arc<String>> {
        if let Some(words) = self.get(key) {
            let word = words.get_random();
            return word;
        }
        None
    }
}

impl GetRandom for HashSet<Arc<String>> {
    fn get_random(&self) -> Option<Arc<String>> {
        let mut rng = rand::thread_rng();
        let words_vec: Vec<Arc<String>> = self.iter().cloned().collect();
        if let Some(word) = words_vec.choose(&mut rng) {
            return Some(Arc::clone(word));
        }
        None
    }
}

impl WordIndex {
    pub fn new() -> WordIndex {
        WordIndex {
            unigrams: HashMap::new(),
            bigrams: HashMap::new(),
        }
    }

    fn add_word(&mut self, word: &str) {
        let word = Arc::new(word.to_string());
        for c in word.chars() {
            let c = c.to_string();
            let unigrams = self.unigrams.entry(c).or_insert(HashSet::new());
            unigrams.insert(word.clone());
        }

        for bigram in word.chars().collect::<Vec<char>>().windows(2) {
            let bigram = bigram.iter().collect::<String>();
            let bigrams = self.bigrams.entry(bigram).or_insert(HashSet::new());
            bigrams.insert(word.clone());
        }
    }

    pub fn read_words(&mut self, words: &str) {
        //split on newline
        for word in words.split_whitespace() {
            self.add_word(word);
        }
    }

    fn get_unigrams(&self, unigram: &str) -> Option<&HashSet<Arc<String>>> {
        self.unigrams.get(unigram)
    }

    fn get_bigrams(&self, bigram: &str) -> Option<&HashSet<Arc<String>>> {
        self.bigrams.get(bigram)
    }

    fn get_random_unigram_word(&self) -> Option<Arc<String>> {
        self.unigrams.get_random()
    }

    fn get_random_bigramm_word(&self) -> Option<Arc<String>> {
        self.bigrams.get_random()
    }

    fn get_word_from_bigramm(&self, bigram: &str) -> Option<Arc<String>> {
        self.bigrams.get_random_from_key(bigram)
    }

    fn get_word_from_unigram(&self, unigram: &str) -> Option<Arc<String>> {
        self.unigrams.get_random_from_key(unigram)
    }

    fn get_random_word(&self) -> Option<Arc<String>> {
        let mut rng = rand::thread_rng();
        //this is kinda unnecessary, since they contain the same words, but who knows
        let word = [&self.unigrams, &self.bigrams]
            .choose(&mut rng)
            .unwrap()
            .get_random();
        word
    }

    pub fn generate_random_lesson(&self, length: usize) -> String {
        let mut lesson = Vec::new();
        let mut lesson_len = 0;
        while lesson_len < length {
            let word = self.get_random_word();
            if let Some(word) = word {
                lesson_len += word.len();
                lesson.push(word);
            }
        }
        let lesson_string = lesson
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ");
        lesson_string
    }
}

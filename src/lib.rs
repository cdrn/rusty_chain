// We're trying to write a generative text markov chain here, with text
// ingested through the command line

use std::fs; // File system manipulation operations
use std::error::Error; // Error utils (result)
use std::collections::HashSet;
use std::collections::HashMap;


// This method should ingest a file and return the contents as a string
pub fn ingest_file (filepath: &String) -> Result<String, Box<dyn Error>> { // this is a string returned wrapped in a "result"
    //Read in a file at the passed path argument
    println!("Our args are {:?}", filepath);
    let ingested_text = fs::read_to_string(filepath)?;
    Ok(ingested_text)
}

// Tokenise a string on whitespace and periods.
pub fn tokenise_string (string: &String) -> Vec<&str> {
    let mut split_on_punctuation = HashSet::new();
    split_on_punctuation.insert(' ');
    split_on_punctuation.insert('.');
    split_on_punctuation.insert('\n');
    split_on_punctuation.insert('\r');
    split_on_punctuation.insert(',');

    // I still don't really understand ownership and the borrow checker
    let tokens: Vec<&str> = string.rsplit(|token| 
    split_on_punctuation.contains(&token)).filter(|x| x != &"").collect();
    tokens
}

// This is meant to encode the datastructures for a vocabulary based on an individual word
// This is essentially a wrapper on the "list" of words, and the count of said words
#[derive(Debug)]
struct Vocabulary {
    word_list: HashMap<String, u64>, // Word_list is a hashmap for easy insertion when counting
    word_count: u64, // TOTAL word count.
}

pub struct LanguageModel {
    markov_chain: HashMap<String, Vocabulary>,
}

impl LanguageModel {
    // Constructor for a new language model
    pub fn new () -> LanguageModel {
        let markov_chain: HashMap<String, Vocabulary> = HashMap::new();
        LanguageModel { markov_chain }
    }
    // Create a probabilistic data structure which encodes a markov decision process
    pub fn generate_markov_chain (mut self, tokenised_words: &Vec<&str>) {
        for (i, token) in tokenised_words.iter().enumerate() {
            println!("our word is {}", token);
            // This isn't a simple hashmap, we'll have to construct an entry from structs if it's not present
            if !self.markov_chain.contains_key(token.clone()) {
                // Get our vocabulary for our given word
                self.markov_chain.insert(token.to_string(), Vocabulary {word_list: HashMap::new(), word_count: 0});
            }
            // Add then next word (if it exists) to the vocabulary for our given word, increment the word count
            if i + 1 < tokenised_words.len() {
                let vocab = self.markov_chain.get_mut(&token.to_string());
                let word_list = &mut vocab.unwrap().word_list;
                let next_word = tokenised_words[i + 1];
                // Insert our next word into the word list
                let entry = word_list.entry(next_word.to_string()).or_insert(0);
                *entry += 1;
                let word_count = &mut self.markov_chain.get_mut(&token.to_string()).unwrap().word_count;
                *word_count += 1;
            }

        }
        println!("Our constructed markov chain is {:?}", self.markov_chain)
    }
    pub fn create_text(self) -> String {
        
    }
}
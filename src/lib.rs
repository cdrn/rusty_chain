// We're trying to write a generative text markov chain here, with text
// ingested through the command line

use std::fs; // File system manipulation operations
use std::error::Error; // Error utils (result)
use std::collections::HashSet;


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

    let tokens: Vec<&str> = string.rsplit(|token| 
    split_on_punctuation.contains(&token)).filter(|x| x != &"").collect();
    tokens
}
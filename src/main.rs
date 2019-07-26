use std::env; // Environment interaction package
use std::process;

use rusty_chain::ingest_file;
use rusty_chain::tokenise_string;

fn main() {
    // We have to ingest our args in main
    let args: Vec<String> = env::args().collect();
    println!("Our args are {:?}", args);
    // Now we can use our first arg as a filepath to ingest a text file
    let file_path: String = args[1].clone();
    let ingested_text = ingest_file(&file_path).unwrap_or_else(|err| {
            println!("something went horribly wrong {}", err);
            // We can either return a string here or shut down the program
            // We're going to choose the latter
            process::exit(1) // exit with code 1
        });

    println!("we have ingested the text {:?}", ingested_text);
    // Once we have our ingested text, we need to tokenise it
    let tokenised_string: Vec<&str> = tokenise_string(&ingested_text);
    println!("Our tokenised string looks like... {:?}", tokenised_string);

    // Now we want to construct a markov chain (Trie?) based on these outputs
    
}

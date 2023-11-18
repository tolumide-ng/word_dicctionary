use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};

use serde_json::json;



pub struct LetterWordCount {
    /// count is the total word length. i.e length of all the words in our dic
    pub count: usize, 
    pub letter_count: HashMap<String, usize>,
}


fn main() {

    fn sample_occurence(cells: usize) {
        let LetterWordCount { count, letter_count } = get_count();
    

    let map = scale_down(cells, count, letter_count);
        println!("THE RESULT {:#?}", map);
    }



    // calculate_probability();
    sample_occurence(112);
}

// fn adjusted_scale_down(cell_size: usize, count: usize, letter_count: HashMap<String, usize>) -> HashMap<String, usize> {
//     let mut map: HashMap<String, usize> = HashMap::from_iter(('A'..='Z').map(|l| (l.to_string(), 0)).collect::<Vec<_>>());

//     for(key, l_count) in letter_count {
//         let adjusted_total_count = (count + 1) * (cell_size -1);
//         let result = 1/adjusted_total_count;
//         // let adjusted_probability = (l_count + 1) as f32/ (count + cell_size) as f32;
//         // let result = adjusted_probability.ceil();

//         let count = map.get_mut(&key).unwrap();
//         *count = result as usize;
//     }

    
//     return map;
// }


fn scale_down(cells: usize, count: usize, letter_count: HashMap<String, usize>) -> HashMap<String, i8> {
        let mut map: HashMap<String, i8> = HashMap::from_iter(('A'..='Z').map(|l| (l.to_string(), 0)).collect::<Vec<_>>());
        for (key, l_count) in letter_count {
            let probability: f32 = l_count as f32 / count as f32;

            let fraction_of_cells = probability * (cells as f32);

            let count = map.get_mut(&key).unwrap();
            *count = fraction_of_cells as i8;
        }

        return map
}


   pub fn get_count() -> LetterWordCount {
        let english_letters_initial = ('A'..='Z').map(|l| (l.to_string(), 0)).collect::<Vec<_>>();

        let mut letter_count: HashMap<String, usize> = HashMap::from_iter(english_letters_initial);
        let mut total_letters = 0;
        

        let base_path = std::env::current_dir().unwrap();
        let words = base_path.join("wordlist.txt");
        let file = File::open(words).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line {
                total_letters += word.len();

                let letters = word.split_terminator("").skip(1).collect::<Vec<_>>();
                for letter in letters {
                    let count = letter_count.get_mut(letter).unwrap();
                    *count +=1;
                }
            }
        }

        // let result = json!({
        //     "total_count": total_letters,
        //     "individual_count": letter_count,
        // }).to_string();

        // let mut file = std::fs::OpenOptions::new().append(true).open(base_path.join("count.json")).unwrap();
        // let _ = file.write(result.as_bytes());

        return LetterWordCount {
            count: total_letters,
            letter_count: letter_count
        };

    }


/// Cleans up pre_wordlist.txt to generate wordlist.txt
pub fn adapt_wordlink() {
    let file_name = "pre_wordlist.txt";

    let base_path = std::env::current_dir().unwrap();
    let words = base_path.join(file_name);
    let file = File::open(words).unwrap();
    let reader = BufReader::new(file);

    let mut newly_formed: Vec<String> = vec![];
    for line in reader.lines() {
        if let Ok(word) = line {
            // let word_up = word.to_uppercase();
            let word = word.split("\"").collect::<Vec<_>>();
            let word_up = word[1].to_uppercase();
            // println!(":::::::::::::::::: {word_up:?}");
            if !newly_formed.contains(&word_up) {
                newly_formed.push(word_up.clone());
            }
        }
    }

    newly_formed.sort();

    for word in newly_formed {
        let mut file = std::fs::OpenOptions::new().append(true).open(base_path.join("wordlist.txt")).unwrap();
        let _ = file.write(format!("{word}\n" ).as_bytes());
    }
}



/// merges words.txt and words_alpha.txt
pub fn merger() {
        let dictionaries = vec!["words.txt", "words_alpha.txt"];

    let base_path = std::env::current_dir().unwrap();

    let mut newly_formed: Vec<String> = vec![];
    for file_name in dictionaries {
        let words = base_path.join(file_name);
        let file = File::open(words).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line {
                let word_up = word.to_uppercase();
                if !newly_formed.contains(&word_up) {
                    newly_formed.push(word_up.clone());
                }
            }
        }
    }

    newly_formed.sort();

    for word in newly_formed {
        let mut file = std::fs::OpenOptions::new().append(true).open(base_path.join("all_words.txt")).unwrap();
        // let ff = file.write(word.as_bytes());
        let ff = file.write(format!("{word}\n" ).as_bytes());
    }

}
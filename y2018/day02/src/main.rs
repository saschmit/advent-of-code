use std::collections::HashMap;

fn get_word_counts(word : &str) -> (bool, bool) {
    let mut counts = HashMap::new();
    for letter in word.chars() {
        let count = counts.entry(letter).or_insert(0);
        *count += 1;
    }

    let mut output = (false, false);
    for (_letter, count) in &counts {
        if *count == 2 {
            output.0 = true;
        } else if *count == 3 {
            output.1 = true;
        }
    }

    output
}

fn main() {
    assert_eq!(get_word_counts("abcdef"), (false, false));
    assert_eq!(get_word_counts("bababc"), (true, true));
    assert_eq!(get_word_counts("abbcde"), (true, false));
    assert_eq!(get_word_counts("abcccd"), (false, true));
    assert_eq!(get_word_counts("aabcdd"), (true, false));
    assert_eq!(get_word_counts("abcdee"), (true, false));
    assert_eq!(get_word_counts("ababab"), (false, true));

    let buff = String::from_utf8(std::fs::read("input").unwrap()).unwrap();
    let words : Vec<&str> = buff.lines().collect();

    let mut twos = 0;
    let mut threes = 0;
    for word in words {
        let (had_two, had_three) = get_word_counts(&word);
        if had_two {
            twos += 1;
        }
        if had_three {
            threes += 1;
        }
    }

    let checksum = twos * threes;
    println!("checksum: {}", checksum);
}

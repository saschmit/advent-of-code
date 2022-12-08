fn diff_words(word1 : &str, word2 : &str) -> Option<String> {
    assert_eq!(word1.len(), word2.len());
    assert!(word1.is_ascii());
    assert!(word2.is_ascii());
    let mut mismatches = 0;
    let mut common_letters = String::new();
    for n in 0..word1.len() {
        if word1.get(n..n+1) != word2.get(n..n+1) {
            mismatches += 1;
        } else {
            match word1.get(n..n+1) {
                Some(s) => common_letters.push_str(s),
                None => unreachable!()
            };
        }
    }
    if mismatches == 1 {
        Some(common_letters)
    } else {
        None
    }
}

fn main() {
    let buff = String::from_utf8(std::fs::read("input").unwrap()).unwrap();
    let words : Vec<&str> = buff.lines().collect();

    for word1 in &words {
        for word2 in &words {
            match diff_words(&word1, &word2) {
                Some(common_letters) => {
                    println!("Common letters: {}", common_letters);
                    return;
                },
                None => {
                }
            }
        }
    }
}

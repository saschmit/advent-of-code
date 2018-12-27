fn main() {
    let file = std::env::args().nth(1).unwrap();
    let buff = std::fs::read(file).unwrap();
    assert_eq!(buff.len() % 49, 0);

    let mut all_steps = std::collections::HashSet::new();
    let mut pairs = Vec::new();
    for i in 0..buff.len() / 49 {
        let (a, b) = (buff[i*49 + 5] as char, buff[i*49 + 36] as char);
        pairs.push((a, b));
        all_steps.insert(a);
        all_steps.insert(b);
    }

    let mut steps_taken = Vec::<char>::new();
    let mut candidates = std::collections::HashSet::new();
    loop {

        // add any step we haven't already taken
        for step in &all_steps {
            if steps_taken.iter().find(|&&x| x == *step) != None {
                continue;
            }
            candidates.insert(*step);
        }

        // remove any step with a prerequisite not already taken
        for (a, b) in &pairs {
            if steps_taken.iter().find(|&&x| x == *a) != None {
                continue;
            }
            if candidates.iter().find(|&&x| x == *b) != None {
                candidates.remove(b);
            }
        }

        // stop if we're out ouf candidates
        if candidates.len() == 0 {
            break;
        }

        use std::iter::FromIterator;
        let mut sorted = Vec::from_iter(&candidates);

        // sort the candidates alphabetically
        sorted.sort();

        // take the first
        steps_taken.push(*sorted[0]);

        // start again
        candidates.clear();
    };

    print!("Steps taken: ");
    for step in steps_taken {
        print!("{}", step);
    }
    println!("");
}

pub fn show_reverse_line(line: String) {
    let line = line.trim_end();
    let mut symbol_positions: Vec<(i32, &str)> = Vec::new();
    let mut line_reversed: Vec<String> = Vec::new();

    let splitting_symbols = &[',', '.', ':'][..];
    let sentence_enders = &['.', '?', '!', ';'][..];

    let line_words_and_symbols_reversed = line.rsplit(' ').collect::<Vec<&str>>();
    for (word_index, word) in line_words_and_symbols_reversed.iter().enumerate() {
        if word.ends_with(sentence_enders) {
            // If we have more than one symbol positions,
            // Then we can now insert the symbol found before right here.
            if symbol_positions.len() >= 1 {
                match symbol_positions.pop() {
                    Some((_pos, symbol)) => {
                        line_reversed.push(symbol.to_string());
                    }
                    None => {}
                }
            }

            let symbol_start_index = word.find(sentence_enders).unwrap();
            let symbol = word.get(symbol_start_index..word.len()).unwrap();
            let word_without_symbol = word.get(0..symbol_start_index).unwrap();

            symbol_positions.push((word_index as i32, symbol));
            line_reversed.push(word_without_symbol.to_string());
        } else if word.ends_with(splitting_symbols) {
            let symbol_start_index = word.find(splitting_symbols).unwrap();
            let symbol = word.get(symbol_start_index..word.len()).unwrap().to_owned();
            let word_without_symbol = word.get(0..symbol_start_index).unwrap();
            let word_with_symbol_in_front = symbol + word_without_symbol;
            line_reversed.push(word_with_symbol_in_front);
        } else {
            line_reversed.push(word.to_string());
        }
    }

    if symbol_positions.len() == 1 {
        match symbol_positions.pop() {
            Some((_pos, symbol)) => {
                line_reversed.push(symbol.to_string());
            }
            None => {}
        }
    }

    println!("Yoda says:");
    for (index, word) in line_reversed.iter().enumerate() {
        if index < line_reversed.len() - 1 {
            print!("{} ", word);
        } else {
            print!("{}", word);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut _should_continue = true;
    let mut user_input = String::new();
    match std::io::stdin().read_line(&mut user_input) {
        Ok(_bytes_read) => {
            show_reverse_line(user_input);
        }
        Err(error) => {
            println!("Failed to read from standard input:{}", error);
            println!("Ending the game.");
            _should_continue = false;
        }
    }
}

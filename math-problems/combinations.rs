fn letter_combinations(digits: &str) -> Vec<String> {
    // If the input is empty, immediately return an empty answer array
    if digits.is_empty() {
        return Vec::new();
    }

    // Map all the digits to their corresponding letters
    let letters = [
        ('2', "abc"), ('3', "def"), ('4', "ghi"), ('5', "jkl"),
        ('6', "mno"), ('7', "pqrs"), ('8', "tuv"), ('9', "wxyz"),
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    fn backtrack(index: usize, path: &mut Vec<char>, digits: &str, letters: &std::collections::HashMap<char, &str>, combinations: &mut Vec<String>) {
        // If the path is the same length as digits, we have a complete combination
        if path.len() == digits.len() {
            combinations.push(path.iter().collect());
            return; // Backtrack
        }

        // Get the letters that the current digit maps to, and loop through them
        let digit = digits.chars().nth(index).unwrap();
        let possible_letters = letters[&digit].chars();
        for letter in possible_letters {
            // Add the letter to our current path
            path.push(letter);
            // Move on to the next digit
            backtrack(index + 1, path, digits, letters, combinations);
            // Backtrack by removing the letter before moving onto the next
            path.pop();
        }
    }

    // Initiate backtracking with an empty path and starting index of 0
    let mut combinations = Vec::new();
    backtrack(0, &mut Vec::new(), digits, &letters, &mut combinations);
    combinations
}

// main
fn main() {
    let digits = "23";
    let combinations = letter_combinations(digits);
    println!("{:?}", combinations);
}
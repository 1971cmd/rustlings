// TODO: This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was instead of just returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Change
// the function signature and body to return `Result<String, String>` instead
// of `Option<String>`.
// This function generates text for a nametag but returns an error if the name is empty.
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Return an error message instead of None
        Err("Empty names aren't allowed".to_string())
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // Example usage
    match generate_nametag_text("Alice".to_string()) {
        Ok(text) => println!("{}", text),
        Err(err) => println!("Error: {}", err),
    }

    match generate_nametag_text("".to_string()) {
        Ok(text) => println!("{}", text),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()),
            Ok("Hi! My name is Beyoncé".to_string()),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new()),
            Err("Empty names aren't allowed".to_string()),
        );
    }
}

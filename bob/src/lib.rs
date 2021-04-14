pub fn reply(message: &str) -> &str {
    let message = message.trim_end(); // Get rid of trailing whitespace.

    if message.is_empty() {
        // Is he addressed with empty space only?
        return "Fine. Be that way!";
    } else if message.contains(|a: char| a.is_alphabetic()) // Check that it contains non-numerical values, 
                                            // otherwise checking if it's in uppercase below does not work. 
        && message.to_uppercase() == message // Is it uppercase?
        && message.ends_with('?')
    {
        return "Calm down, I know what I'm doing!";
    } else if message.contains(|a: char| a.is_alphabetic()) && message.to_uppercase() == message {
        return "Whoa, chill out!";
    } else if message.ends_with('?') {
        return "Sure.";
    } else {
        return "Whatever.";
    }
}

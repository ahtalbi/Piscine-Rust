enum Message {
    Yelling,
    Asking,
    YellingQuestion,
    Nothing,
    Interesting,
}

fn classify_message(s: &str) -> Message {
    // check if nothing
    if s.is_empty() { return Message::Nothing; }
    
    // check if yelling
    let has_letter = s.chars().any(|c| c.is_alphabetic());
    let is_yaelling = has_letter && s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) && !s.ends_with('?');
    if is_yelling { return Message::Yelling; }

    // check if he is yelling a question
    let is_yelling_question = s.chars().all(|c| !c.is_alphabetic() || c.is_ascii_uppercase()) && s.ends_with('?');
    if is_yelling_question { return Message::YellingQuestion; }

    // check if is asking normal
    let is_asking = &s[s.len()-1..s.len()] == "?";
    if is_asking { return Message::Asking; }

    Message::Interesting
}

pub fn talking(text: &str) -> &str {
    let classified : Message = classify_message(&text);
    match classified {
        Message::Nothing => {
            "Just say something!"
        },
        Message::Yelling => {
            "There is no need to yell, calm down!"
        },
        Message::Asking => {
            "Sure."
        },
        Message::YellingQuestion => {
            "Quiet, I am thinking!"
        },
        Message::Interesting => {
            "Interesting"
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", talking("JUST DO IT!"));
        println!("{:?}", talking("Hello how are you?"));
        println!("{:?}", talking("WHAT'S GOING ON?"));
        println!("{:?}", talking("something"));
        println!("{:?}", talking(""));
    }
}

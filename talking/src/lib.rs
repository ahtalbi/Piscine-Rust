enum Message {
    Yelling,
    Asking,
    YellingQuestion,
    Nothing,
    Interesting,
}

fn classify_message(s: &str) -> Message {
    let is_empty = s.is_empty();
    let has_letter = s.chars().any(|c| c.is_alphabetic());
    let all_upper = has_letter && s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    let is_question = s.ends_with('?');

    match (is_empty, all_upper, is_question) {
        (true, _, _) => Message::Nothing,
        (false, true, true) => Message::YellingQuestion,
        (false, true, false) => Message::Yelling,
        (false, false, true) => Message::Asking,
        _ => Message::Interesting,
    }
}

pub fn talking(text: &str) -> &str {
    println!("{:?}", text);
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

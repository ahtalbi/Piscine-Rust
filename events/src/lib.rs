use colored::*;
use std::{fmt, time::Duration};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.color;
        let c = self.content.truecolor(r, g, b);
        write!(f, "({:?}, {}, {})", self.position, self.size, c)
    }
}

impl Event<'_> {
    pub fn notify(self) -> Notification {
        match self {
            Event::Remainder(s) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: s.to_string(),
            },
            Event::Registration(d) => {
                let s = d.as_secs();
                let res = format!(
                    "You have {}H:{:02}M:{:02}S left before the registration ends",
                    s / 3600, (s % 3600) / 60, s % 60
                );
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: res,
                }
            }
            Event::Appointment(s) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: s.to_string(),
            },
            _ => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", Event::Remainder("Go to the doctor").notify());
        println!(
            "{}",
            Event::Registration(Duration::from_secs(49094)).notify()
        );
        println!("{}", Event::Appointment("Go to the doctor").notify());
        println!("{}", Event::Holiday.notify());
    }
}

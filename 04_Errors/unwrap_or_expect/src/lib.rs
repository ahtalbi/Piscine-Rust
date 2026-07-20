pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server : Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            server.unwrap().to_string()
        },
        Security::Message => {
            match server {
                Ok(url) => {
                    return url.to_string();
                },
                Err(_) => {
                    panic!("ERROR: program stops");
                },
            }
        },
        Security::Warning => {
            match server {
                Ok(url) => {
                    return url.to_string();
                },
                Err(_) => {
                    return "WARNING: check the server".to_string();
                },
            }
        },
        Security::NotFound => {
            match server {
                Ok(url) => {
                    return url.to_string();
                },
                Err(e) => {
                    return format!("Not found: {}", e);
                },
            }
        },
        Security::UnexpectedUrl => {
            match server {
                Ok(url) => {
                    panic!("{}", url);
                },
                Err(e) => {
                    return e.to_string();
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
        fn it_works() {
        println!("{}", fetch_data(Ok("server1.com"), Security::Warning));
        println!("{}", fetch_data(Err("server.com"), Security::Warning));
        println!("{}", fetch_data(Err("server2.com"), Security::NotFound));

        // Panics with no custom message
        // fetch_data(Err("ERROR CRITICAL"), Security::Unknown);

        // Panics with the message "ERROR: program stops"
        // fetch_data(Err("server.com"), Security::Message);

        // Panics with the message "malicious_server.com"
        // fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }
}

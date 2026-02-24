# Secrecy Example

This is a very simple implementation of the `secrecy` crate to get started.

The idea is that the values put into the `Password` struct are easy to use for debugging / logging but you can't easily expose the secrets.

```rust
use std::fmt::{Debug, Display, Formatter, Result};

use secrecy::{ExposeSecret, SecretBox};

#[derive(Debug)]
struct Password {
    value: SecretBox<String>
}

impl Password {
    pub fn new(value: String) -> Self {
        Self {value: SecretBox::new(Box::new(value))}
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if cfg!(debug_assertions) {
            write!(f, "{}", self.value.expose_secret())
        }
        else {
            write!(f,"REDACTED")
        }
    }
}

fn main() {

    let basic_password: SecretBox<String> = SecretBox::new(Box::new(String::from("pass")));
    println!("Manually typed password printed with expose_secret(): \"{}\"", basic_password.expose_secret());

    /*  
        This method would easily allow you to create logs with usernames / passwords visible for debugging
        but have confidence that they wouldn't get printed in release builds.
    */
    let custom_typed_password:Password = Password::new("test".to_string());
    println!("Custom struct password printed with traits: \"{}\"", custom_typed_password);
}
```
use chrono::Local;
use std::fs::File;
use std::io::{self, Write}; // Import Write to use write! macro

fn main() -> io::Result<()> {
    let mut file = File::create("output.txt")?; // Try to create or open file

    let header = "Tommy H. Yeargin, Jr.\n1014 Circle Rd.\nEasley, SC 29642";
    println!("{}", header);
    write!(file, "{}\n", header)?; // Write header to file

    let today = Local::now();
    let date_str = format!("\n{}", today.format("%d %B %Y"));
    print!("{}", date_str);
    write!(file, "{}", date_str)?; // Write date to file

    let recipient_address = r#"

Recipient Name
Recipient Address
Recipient City, State Zip Code

Dear Recipient Name,

Body of message...

Sincerely,


Tommy H. Yeargin, Jr.

tyjr
"#;
    println!("{}", recipient_address);
    write!(file, "{}", recipient_address)?; // Write address and message to file

    Ok(())
}
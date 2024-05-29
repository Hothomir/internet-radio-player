use rodio::{Decoder, Sink, Source};
use std::io::{Error, ErrorKind, Read};
use reqwest;

fn main() -> Result<(), Error> {
    
    // Create a new string to be used
    let mut url = String::new();

    // Print message for user
    println!("Enter the radio stream URL: ");

    // Take in new url from user
    std::io::stdin().read_line(&mut url)?;
    url.pop();

    // Include a station name
    let mut station_name = String::new();
    println!("Enter the radio name: ");
    std::io::stdin().read_line(&mut station_name)?;

    // Open the URL as a source
    let source = Decoder::new(reqwest::get(url)?.error_for_status()?.bytes())?;

    // Get the default output device
    let sink = Sink::new(&rodio::default_output_config())?;

    // Append the source to the sink and start playing
    sink.append(source);

    // Keep the program running until interrupted (Ctrl+C)
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

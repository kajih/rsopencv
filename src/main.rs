use opencv::prelude::*;
use opencv::{highgui, imgcodecs, Result};
use opencv::core::get_version_string;

fn main() -> Result<()> {
    let foo = get_version_string();

    println!("V {:?}", foo);
    
    let image = imgcodecs::imread("Ankha.jpg", 0)?;
    highgui::named_window("hello opencv!", 0)?;
    highgui::imshow("hello opencv!", &image)?;
    highgui::wait_key(10000)?;

    Ok(())
}


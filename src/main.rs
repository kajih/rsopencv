use opencv::core::get_version_string;
use opencv::prelude::*;
use opencv::{highgui, imgcodecs, videoio, Result};

fn main() -> Result<()> {
    let foo = get_version_string();

    println!("V {:?}", foo);
    highgui::named_window("hello opencv!", 0)?;
    let image = imgcodecs::imread("Ankha.jpg", 0)?;
    highgui::imshow("hello opencv!", &image)?;

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default(); // This array will store the web-cam data

    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame)?;
        let key = highgui::wait_key(1)?;
        if key == 113 {
            // quit with q
            break;
        }
    }
    Ok(())
}

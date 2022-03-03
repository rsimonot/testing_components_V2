use std::error::Error;

use rppal::gpio::Gpio;

const PHOTO_PIN: u8 = 6;

fn get_value_photosensor() -> Result<(), Box<dyn Error>> {
    let pin = Gpio::new()?.get(PHOTO_PIN)?.into_input();
    if pin.read() == rppal::gpio::Level::Low {
        println!("> Stuff detected");
    } else {
        println!("> Nothing detected");
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    get_value_photosensor()?;

    Ok(())
}
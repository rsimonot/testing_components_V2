use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const FAN_PIN: u8 = 18;

fn trigger_fan() -> Result<(), Box<dyn Error>> {
    println!("Testing the fan on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(FAN_PIN)?.into_output();
    pin.set_high();
    thread::sleep(Duration::from_millis(2500));
    pin.set_low();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    trigger_fan()?;

    Ok(())
}
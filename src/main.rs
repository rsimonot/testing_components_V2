use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;
use rppal::i2c::I2c;

const FAN_PIN: u8 = 18;
const PHOTO_PIN: u8 = 6;

const TMP116_ADR: u16 = 0x48;
const TEMP_REG: u8 = 0x00;

const OE_PIN: u8 = 7;
const PCA9624PW_ADR: u16 = 0x15;
const MODE1_REG: u8 = 0x00;
const LEDOUT0_REG: u8 = 0x0c;
const LEDOUT1_REG: u8 = 0x0d;
const LED1_REG: u8 = 0x02;
const LED2_REG: u8 = 0x03;
const LED3_REG: u8 = 0x04;
const LED4_REG: u8 = 0x05;
const LED5_REG: u8 = 0x06;
const LED6_REG: u8 = 0x07;
const LED7_REG: u8 = 0x08;

fn trigger_fan() -> Result<(), Box<dyn Error>> {
    println!("Testing the fan on a {}.", DeviceInfo::new()?.model());

    let mut pin = Gpio::new()?.get(FAN_PIN)?.into_output();
    pin.set_high();
    thread::sleep(Duration::from_millis(2500));
    pin.set_low();

    Ok(())
}

fn get_value_photosensor() -> Result<(), Box<dyn Error>> {
    let pin = Gpio::new()?.get(PHOTO_PIN)?.into_input();
    println!("Photosensor pin value is : {}", pin.read());
    
    Ok(())
}

fn i2c_get_temperature() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(TMP116_ADR)?;

    i2c.write(&[TEMP_REG])?;

    //let mut buffer = [0u8; 1];  // array of size 1 containing 0 as u8 value
    //i2c.read(&mut buffer)?;

    let temp = i2c.smbus_read_word(TEMP_REG)?;
    println!("Read temperature => {}°C", (temp>>8|((temp<<8)&0xffff)) as f64 * 0.0078125);

    Ok(())
}

fn manage_leds() -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(OE_PIN)?.into_output();
    pin.set_low();

    let mut i2c = I2c::new()?;
    i2c.set_slave_address(PCA9624PW_ADR)?;

    // Parameters initialization
    i2c.block_write(MODE1_REG, &[0x00])?;
    i2c.block_write(LEDOUT0_REG, &[0xff])?;
    i2c.block_write(LEDOUT1_REG, &[0xff])?;

    // LEDs one by one
    i2c.block_write(LED1_REG, &[0xff])?;
    i2c.block_write(LED2_REG, &[0xff])?;
    i2c.block_write(LED3_REG, &[0xff])?;
    i2c.block_write(LED4_REG, &[0xff])?;
    i2c.block_write(LED5_REG, &[0xff])?;
    i2c.block_write(LED6_REG, &[0xff])?;
    i2c.block_write(LED7_REG, &[0xff])?;
    thread::sleep(Duration::from_millis(2500));

    pin.set_high();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    //trigger_fan()?;
    //get_value_photosensor()?;
    //i2c_get_temperature()?;
    manage_leds()?;

    Ok(())
}
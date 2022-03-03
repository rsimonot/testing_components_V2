use std::error::Error;

use rppal::i2c::I2c;

const TMP116_ADR: u16 = 0x48;
const TEMP_REG: u8 = 0x00;

fn i2c_get_temperature() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(TMP116_ADR)?;

    i2c.write(&[TEMP_REG])?;

    let temp = i2c.smbus_read_word(TEMP_REG)?;
    println!("> Read temperature => {}°C", (temp>>8|((temp<<8)&0xffff)) as f64 * 0.0078125);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    i2c_get_temperature()?;

    Ok(())
}
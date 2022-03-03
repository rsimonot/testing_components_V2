use std::error::Error;

use rppal::i2c::I2c;

const BAT_ADR: u16 = 0x48;
const SOC_REG: u8 = 0x06;

fn i2c_get_current_charge() -> Result<(), Box<dyn Error>> {
    let mut i2c = I2c::new()?;
    i2c.set_slave_address(BAT_ADR)?;

    i2c.write(&[SOC_REG])?;

    let charge = i2c.smbus_read_word(SOC_REG)?;
    println!("> Read Current Charge => {}%", charge);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    i2c_get_current_charge()?;

    Ok(())
}
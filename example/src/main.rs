use AFE4404::AFE4404;
use esp_idf_hal::{prelude::*, peripherals::Peripherals, i2c::{Master, MasterPins, config::MasterConfig}};

fn main() {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let config = MasterConfig::new()
        .baudrate(400.kHz().into());

    let i2c = Master::new(
        peripherals.i2c0,
        MasterPins {
            sda: peripherals.pins.gpio3,
            scl: peripherals.pins.gpio2,
        },
        config,
    ).expect("Failed to initialize I2C bus.");

    let frontend = AFE4404::new(i2c, 0x58u8.into());

    println!("Hello, world!");
}

use esp_idf_hal::delay::Ets;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c;
use esp_idf_hal::units::FromValueType;


pub fn get_i2c() -> Result<i2c::I2cDriver<'static>, esp_idf_hal::sys::EspError> {

    let peripherals = unsafe { Peripherals::new() } ;
    
    //   take().unwrap_or_else(|e| {
    //     panic!("i2c peripheral tack failed: {}", e.to_string());
    // } );
    let config = i2c::config::Config::new().baudrate(400u32.kHz().into());

    i2c::I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio21, // sda
        peripherals.pins.gpio22, // scl
        &config
    )
}

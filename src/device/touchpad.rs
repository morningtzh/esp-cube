use crate::device::i2c::get_i2c;
use esp_idf_hal::delay;
use esp_idf_hal::delay::Delay;
use esp_idf_hal::i2c;

use esp_idf_svc::mqtt::client::Event;
use ft6x36;

pub struct TouchPad {
    device: ft6x36::Ft6x36<i2c::I2cDriver<'static>>,
}

impl TouchPad {
    pub fn new() -> Self {
        let i2c = match get_i2c() {
            Ok(i2c) => i2c,
            Err(err) => {
                panic!("can't get i2c: {}", err);
            }
        };

        

        TouchPad {
            device: ft6x36::Ft6x36::new(i2c, ft6x36::Dimension(320, 240)),
        }
    }

    pub fn get_touch_event(&mut self) -> ft6x36::RawTouchEvent {
        match self.device.get_touch_event() {
            Ok(event) => event,
            Err(e) => {
                println!("get touch event failed: {}", e);

                ft6x36::RawTouchEvent {
                    device_mode: ft6x36::DeviceMode::Factory,
                    gesture_id: ft6x36::GestureId::NoGesture,
                    p1: None,
                    p2: None,
                }
            }
        }
    }
}

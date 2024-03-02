use esp_idf_hal::{
    adc::attenuation::NONE,
    i2c::{self, I2cDriver},
};

use crate::device::i2c::get_i2c;
use esp_idf_hal::delay::Ets;
use libm::{atan2f, powf, sqrtf};
use mpu6886::Mpu6886;
use nalgebra::{Vector2, Vector3};

#[derive(Debug)]
pub struct ImuData<T> {
    acc: Option<Vector3<T>>,
    gyro: Option<Vector3<T>>,
    acc_angles: Option<Vector2<T>>,
    temp: Option<T>,
}

impl<T> ImuData<T> {
    pub fn empty() -> Self {
        ImuData::<T> {
            acc: None,
            gyro: None,
            acc_angles: None,
            temp: None,
        }
    }
}

pub(crate) trait ImuTrait<T> {
    fn get_data(&mut self) -> ImuData<T>;
}

static mut MPU: Option<Mpu6886<i2c::I2cDriver<'_>>> = None;

pub(crate) struct Mpu6886Driver<'a> {
    mpu: Mpu6886<I2cDriver<'a>>,
}

impl Mpu6886Driver<'_> {
    pub fn new() -> Self {
        let i2c = match get_i2c() {
            Ok(i2c) => i2c,
            Err(err) => {
                panic!("can't get i2c: {}", err);
            }
        };
        // let bus = shared_bus::BusManagerSimple::new(i2c);
        let mut mpu = Mpu6886::new(i2c);
        let _ret = mpu.init(&mut Ets);

        Mpu6886Driver { mpu: mpu }
    }
}

impl ImuTrait<f32> for Mpu6886Driver<'_> {
    fn get_data(&mut self) -> ImuData<f32> {
        let mut data = ImuData::<f32>::empty();

        // get temperature data
        data.temp = match self.mpu.get_temp() {
            Ok(t) => Some(t),
            Err(e) => {
                log::error!("Mpu6886Driver get temp failed: {:?}", e);
                None
            }
        };

        match self.mpu.get_acc() {
            Ok(acc) => {
                // get accelerometer data, scaled with sensitivity
                data.acc = Some(acc);

                // Roll and pitch estimation from raw accelerometer readings
                // NOTE: no yaw! no magnetometer present on mpu6886
                // https://www.nxp.com/docs/en/application-note/AN3461.pdf equation 28, 29
                data.acc_angles = Some(Vector2::<f32>::new(
                    atan2f(acc.y, sqrtf(powf(acc.x, 2.) + powf(acc.z, 2.))),
                    atan2f(-acc.x, sqrtf(powf(acc.y, 2.) + powf(acc.z, 2.))),
                ));
            }
            Err(e) => {
                log::error!("Mpu6886Driver get acc failed: {:?}", e);                
            }
        };

        // get gyro data, scaled with sensitivity
        data.gyro = match self.mpu.get_gyro() {
            Ok(g) => Some(g),
            Err(e) => {
                log::error!("Mpu6886Driver get gyro failed: {:?}", e);
                None
            }
        };

        data
    }
}

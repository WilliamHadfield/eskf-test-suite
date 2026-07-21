
#![no_std]
#![no_main]

mod I2C;
mod logic;


use eskf_gaunlet_testsuite::shared::{I2C_BUS};
use crate::I2C::mpu6050::{MPU_CHANNEL_LOG, mpu_task};
use embassy_time::Timer;
use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals;
use embassy_stm32::{ dma, i2c};
use embassy_executor::Spawner;
use embassy_stm32::{i2c::I2c, wdg::IndependentWatchdog};
use embassy_stm32::time::Hertz;
use embassy_sync::mutex::Mutex;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use {defmt_rtt as _, panic_probe as _};


bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DMA1_STREAM6 => dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
});






#[embassy_executor::main]
async fn main (spawner : Spawner) {
let mut config = embassy_stm32::Config::default();
   
    let p = embassy_stm32::init(config);


     let mut i2c_config = embassy_stm32::i2c::Config::default();
   i2c_config.frequency = Hertz::hz(400_000);
   i2c_config.timeout = embassy_time::Duration::from_millis(10);
    let mut i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        p.DMA1_CH6,
        p.DMA1_CH0,
        Irqs,
       i2c_config,
    );
let i2c_bus = I2C_BUS.init(Mutex::new(i2c));
let i2c_device_mpu = I2cDevice::new(i2c_bus);
Timer::after_millis(30).await;


    spawner.spawn(mpu_task(i2c_device_mpu).unwrap());
Timer::after_millis(30).await;

}

#[embassy_executor::task]
async fn ReadAllData(){
loop {

Timer::after_millis(80).await;
}
}


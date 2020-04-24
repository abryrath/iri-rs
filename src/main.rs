extern crate gpio;
use gpio::{GpioOut,GpioIn};
use std::{thread, time};

fn main() {
    let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
    let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();
    let mut value = true;
    thread::spawn(move || loop {
        gpio24.set_value(value).expect("Could not set GPIO24");
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
    });
    loop {
        println!("GPIO23: {:?}", gpio23.read_value().unwrap());
        thread::sleep(time::Duration::from_millis(100));
    }
}

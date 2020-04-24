extern crate gpio_cdev;
use gpio_cdev::{Chip, EventRequestFlags, EventType, LineRequestFlags};
use std::{thread, time};

// out - 28 - GPIO 1
// in  - 03 - GPIO 2

fn main() {
    let output: u32 = 28;
    let input: u32 = 3;

    let chip = Chip::new("/dev/gpiochip0").unwrap();
    let input_line = chip.get_line(input).unwrap();
    let output_line = chip.get_line(output).unwrap();
    let output_handle = output_line.request(LineRequestFlags::OUTPUT, 0, "idk").unwrap();
    // let mut value = true;

    for event in input_line.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::BOTH_EDGES,
        "idk",
    ).unwrap() {
        let evt = event.unwrap();
        match evt.event_type() {
            EventType::RisingEdge => {
                println!("Setting value high");
                output_handle.set_value(1).unwrap();
            }
            EventType::FallingEdge => {
                println!("Setting value low");
                output_handle.set_value(0).unwrap();
            }
        }
    }
    // thread::spawn(move || loop {
    //     gpio24.set_value(value).expect("Could not set GPIO24");
    //     thread::sleep(time::Duration::from_millis(1000));
    //     value = !value;
    // });
    // loop {
    //     println!("GPIO23: {:?}", gpio23.read_value().unwrap());
    //     thread::sleep(time::Duration::from_millis(100));
    // }
}

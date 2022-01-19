use rppal::gpio::{Gpio, Level, Trigger};
use std::time::{SystemTime};

const WHEEL_DIAMETER: f64 = 0.5858; //110/70 Front wheel diameter in metres
const WHEEL_CIRCUMFERENCE: f64 = 1.84034; //110/70 Front wheel circumference in metres
const NANO_SECONDS_TO_SECONDS: f64 = 1000.0 * 1000.0 * 1000.0;

/// Returns revs per minute
fn read_tacho(now: u128, last_interrupt_time_tacho: u128) -> f64 {
    let time_taken = (now - last_interrupt_time_tacho) as f64;
    let revs_per_second = time_taken / NANO_SECONDS_TO_SECONDS;
    revs_per_second * 60.0
}

/// Returns kilometres per hour
fn read_speed(now: u128, last_interrupt_time_speed: u128) -> f64 {
    let time_taken = (now - last_interrupt_time_speed) as f64;
    let metres_per_second = (WHEEL_CIRCUMFERENCE / 4.0 / time_taken) * NANO_SECONDS_TO_SECONDS;
    (metres_per_second / 1000.0) * (60.0 * 60.0)
}

fn read_fuel() -> f64 {
    //read I2c address 0
    let out: f64;

    out = 0.0;

    return out;
}

fn read_temp() -> f64 {
    //read I2c address 1
    let out: f64;

    out = 0.0;

    return out;
}

//Setup
struct WebInterface {
    highbeam_indicator: Level,
    left_indicator: Level,
    right_indicator: Level,
    neutral_indicator: Level,
    injection_indicator: Level,
    speed: f64,
    tacho: f64,
    fuel: f64,
    temp: f64,
}

fn main() {
    let mut highbeam_indicator = Level::Low;
    let mut left_indicator = Level::Low;
    let mut right_indicator = Level::Low;
    let mut neutral_indicator = Level::Low;
    let mut injection_indicator = Level::Low;
    let mut fuel: f64 = 0.0;
    let mut temp: f64 = 0.0;

    let mut speed: f64 = 0.0;
    let mut tacho: f64 = 0.0;
    let mut last_interrupt_time_speed: u128 = 0;
    let mut last_interrupt_time_tacho: u128 = 0;

    let gpio = Gpio::new().expect("Failed to acquire GPIO.");
    // let inputpin = InputPin::new(mut pin: Pin, pud_mode: PullUpDown);

    let pin4 = gpio //highbeam pin
        .get(4)
        .expect("Failed to acquire pin 4.")
        .into_input();

    let pin17 = gpio //left turn signal pin
        .get(17)
        .expect("Failed to acquire pin 17.")
        .into_input();

    let pin18 = gpio //right turn signal pin
        .get(18)
        .expect("Failed to acquire pin 18.")
        .into_input();

    let pin27 = gpio //neutral indicator pin
        .get(27)
        .expect("Failed to acquire pin 27.")
        .into_input();

    let pin22 = gpio //fuel injection pin
        .get(22)
        .expect("Failed to acquire pin 22.")
        .into_input();

    let speed_interrupt_handler = {
        move |_| {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
            speed = read_speed(now, last_interrupt_time_speed);
        }
    };
    let _pin23 = gpio //speed pin
        .get(23)
        .expect("Failed to acquire pin 23.")
        .into_input()
        .set_async_interrupt(Trigger::RisingEdge, speed_interrupt_handler); // || refers to variable

    let tacho_interrupt_handler = {
        move |_| {
            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
            tacho = read_tacho(now, last_interrupt_time_tacho);
        }
    };
    let _pin25 = gpio //tachometer pin
        .get(25)
        .expect("Failed to acquire pin 25.")
        .into_input()
        .set_async_interrupt(Trigger::RisingEdge, tacho_interrupt_handler);

    loop {
        highbeam_indicator = pin4.read();
        left_indicator = pin17.read();
        right_indicator = pin18.read();
        neutral_indicator = pin27.read();
        injection_indicator = pin22.read();
        fuel = read_fuel();
        temp = read_temp();

        // Create a struct which represents all current values
        let interface = WebInterface {
            highbeam_indicator,
            left_indicator,
            right_indicator,
            neutral_indicator,
            injection_indicator,
            fuel,
            temp,
            speed,
            tacho,
        };

        // Send this value to the web browser
        // @todo.
    }
}

#[test]
fn test_speed_interrupt_handler() {
    let speed = read_speed((0.01 * NANO_SECONDS_TO_SECONDS) as u128, 0);
    assert_eq!(speed, 46.008500000000005);
}

#[test]
fn test_tacho_interrupt_handler() {
    let tacho = read_tacho((1.0 * NANO_SECONDS_TO_SECONDS) as u128, 0);
    assert_eq!(tacho, 60.0);
}

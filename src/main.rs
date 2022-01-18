use rppal::gpio::{Gpio, Level, Trigger};
//use rppal::i2c::I2c;
use std::time::{SystemTime};
use std::sync::{Arc, Mutex};

const WHEEL_DIAMETER: f64 = 0.5858; //110/70 Front wheel diameter in metres
const WHEEL_CIRCUMFERENCE: f64 = 1.84034; //110/70 Front wheel circumference in metres


/// interrupt on rising edge
/// 
/// save current time to endTime
/// 
/// time = endTime - startTime
/// 
/// RPM = time*60
/// 
/// return RPM
/// 
/// startTime = endTime
/// 
fn tacho(mut inter: &mut Interface) -> f64 {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    let time_taken = (now - inter.last_interrupt_time_tacho) as f64;
    inter.last_interrupt_time_tacho = now;

    ((1000.0 / time_taken) / 60.0) / 1000.0 / 1000.0
}

fn speed(mut inter: &mut Interface) -> f64 {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    let time_taken = (now - inter.last_interrupt_time_speed) as f64;
    inter.last_interrupt_time_speed = now;

    WHEEL_CIRCUMFERENCE / 4.0 / time_taken
}

fn fuel() -> f32 {
    //read I2c address 0
    let out: f32;

    out = 0.0;

    return out;
}

fn temp() -> f32 {
    //read I2c address 1
    let out: f32;

    out = 0.0;

    return out;
}

//Setup
struct Interface {
    highbeam: Level,
    left: Level,
    right: Level,
    neutral: Level,
    injection: Level,
    speed: f64,
    tacho: f64,
    fuel: f32,
    temp: f32,
    last_interrupt_time_speed: u128,
    last_interrupt_time_tacho: u128,
}

fn main() {
    let inter1 = Arc::new(
        Mutex::new(
            Interface {
                highbeam: Level::Low,
                left: Level::Low,
                right: Level::Low,
                neutral: Level::Low,
                injection: Level::Low,
                speed: 0.0,
                tacho: 0.0,
                fuel: 0.0,
                temp: 0.0,
                last_interrupt_time_speed: 0,
                last_interrupt_time_tacho: 0,
            }
        )
    );

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

    let arc_copy_speed = Arc::clone(&inter1);
    let _pin23 = gpio //speed pin
        .get(23)
        .expect("Failed to acquire pin 23.")
        .into_input()
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            let mut guard = arc_copy_speed.lock().unwrap();
            guard.speed = speed(&mut guard);
        }); // || refers to variable

    let arc_copy_tacho = Arc::clone(&inter1);
    let _pin25 = gpio //tachometer pin
        .get(25)
        .expect("Failed to acquire pin 25.")
        .into_input()
        .set_async_interrupt(Trigger::RisingEdge, move |_| {
            let mut guard = arc_copy_tacho.lock().unwrap();
            guard.tacho = tacho(&mut guard);
        });

    loop {
        let mut guard = inter1.lock().unwrap();
        guard.highbeam = pin4.read();
        guard.left = pin17.read();
        guard.right = pin18.read();
        guard.neutral = pin27.read();
        guard.injection = pin22.read();
        guard.fuel = fuel();
        guard.temp = temp();

        //if inter1.highbeam == Level::High {
        println!("High Beam On");
        //}
    }
}
use panic_halt as _;
use arduino_hal::port::{mode::Output, Pin};

pub fn on(led_pin: &mut Pin<Output>){
    led_pin.set_high();
}

pub fn off(led_pin: &mut Pin<Output>){
    led_pin.set_low();
}

pub fn flicker(led_pin: &mut Pin<Output>, n: u8, delay: u16){
    for _ in 0..n {
        led_pin.set_high();
        arduino_hal::delay_ms(delay);
        led_pin.set_low();
        arduino_hal::delay_ms(delay);
    }
}



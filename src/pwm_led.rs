use arduino_hal::simple_pwm::PwmPinOps;
use panic_halt as _;
use arduino_hal::port::Pin;
use arduino_hal::port::mode::PwmOutput;

pub fn set_brightness<TC, P: PwmPinOps<TC>>(pwm_pin: &mut Pin<PwmOutput<TC>, P>, brightness: u8){
    pwm_pin.set_duty(brightness);
}

pub fn fade_on<TC, P: PwmPinOps<TC>>(pwm_pin: &mut Pin<PwmOutput<TC>, P>, delay_ms: u16, start: u8, max: u8){
    for i in start..max {
        set_brightness(pwm_pin, i);
        arduino_hal::delay_ms(delay_ms);
    }
}
pub fn fade_off<TC, P: PwmPinOps<TC>>(pwm_pin: &mut Pin<PwmOutput<TC>, P>, delay_ms: u16, start: u8, min: u8){
    for i in (min..start).rev() {
        set_brightness(pwm_pin, i);
        arduino_hal::delay_ms(delay_ms);
    }
}
pub fn pulse<TC, P: PwmPinOps<TC>>(pwm_pin: &mut Pin<PwmOutput<TC>, P>, delay_ms: u16, min: u8, max: u8, repeats: u16){
    for _ in 0..repeats{
        fade_on(pwm_pin, delay_ms, min, max);
        fade_off(pwm_pin, delay_ms, max, min);
    }
}

pub fn off<TC, P: PwmPinOps<TC>>(pwm_pin: &mut Pin<PwmOutput<TC>, P>){
    pwm_pin.set_duty(0);
}


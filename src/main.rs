// use ableton_link::*;

mod led;
mod lnk;

fn main() {
    let mut link = lnk::init();

    let mut led_control = led::init();

    let leds = led_control.leds_mut(0);

    // Program leds array with BGRA value
    for led in leds {
        *led = [0, 255, 0, 0];
    }

    // Draw
    led_control.render().unwrap();

    loop {}
}

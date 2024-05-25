use std::{thread, time::Duration};

use open_dmx::DMXSerial;

#[inline]
pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    (end - start) * amount + start
}

fn main() {
    let mut dmx = DMXSerial::open("/dev/ttyUSB0").unwrap();

    let mut channels = [0; 40];

    let mut i: f32 = 0.0;

    loop {
        channels.fill(0);

        let before = i as usize;
        let after = before + 1;

        channels[before] = (i.fract() * 255.0) as u8;
        channels[after] = ((1.0 - i.fract()) * 255.0) as u8;

        dmx.set_channels(channels);

        thread::sleep(Duration::from_millis(50));

        i = (i + 0.1) % 24.0;
    }
}

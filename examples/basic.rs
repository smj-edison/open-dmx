use std::{ops::Range, thread, time::Duration};

use open_dmx::DMXSerial;

#[inline]
pub fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    (end - start) * amount + start
}

fn main() {
    let mut dmx = DMXSerial::open("COM4").unwrap();

    let mut channels = [0; 40];

    let mut i: f32 = 0.0;
    let mut counter: usize = 0;

    loop {
        channels.fill(0);

        // let before = i as usize;
        // let after = before + 1;

        // channels[before * 3] = ((1.0 - i.fract()) * 255.0) as u8;
        // channels[after * 3] = (i.fract() * 255.0) as u8;

        // dmx.set_channels(channels);

        // thread::sleep(Duration::from_millis(20));

        // i = (i + 0.1) % 8.0;

        counter += 1;

        if counter <= 8 {
            set_rgb(
                &mut channels,
                0..4,
                0,
                0,
                if counter % 2 == 0 { 255 } else { 0 },
            );
        } else {
            set_rgb(
                &mut channels,
                4..8,
                if counter % 2 == 0 { 255 } else { 0 },
                0,
                0,
            );
        }

        if counter >= 16 {
            counter = 0;
        }

        dmx.set_channels(channels);
        thread::sleep(Duration::from_millis(40));
    }
}

fn set_rgb(channels: &mut [u8], range: Range<usize>, r: u8, g: u8, b: u8) {
    for channel in range {
        channels[channel * 3] = r;
        channels[channel * 3 + 1] = g;
        channels[channel * 3 + 2] = b;
    }
}

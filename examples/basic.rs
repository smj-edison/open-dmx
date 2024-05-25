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

        for sector in 0..8 {
            let offset = ((i + sector as f32) % 8.0) / 8.0;

            let amount = if offset > 0.5 {
                (1.0 - offset) * 2.0
            } else {
                offset * 2.0
            };

            let powah = amount.powf(2.2);

            channels[sector * 3 + 0] = (powah * 255.0) as u8;
        }

        i = (i + 0.1) % 8.0;

        dmx.set_channels(channels);
        thread::sleep(Duration::from_millis(100));
    }
}

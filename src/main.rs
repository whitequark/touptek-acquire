extern crate touptek;
extern crate byteorder;

use std::io::Write;
use std::fs::File;
use byteorder::{WriteBytesExt, BigEndian};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 {
        writeln!(&mut std::io::stderr(),
                 "Usage: touptek-acquire [camera index] [exposure time (ms)] \
                  [exposure gain (%)] [image path]").unwrap();
        std::process::exit(1)
    }

    let camera_index  = args[1].parse::<u32>().expect("Cannot parse camera index!");
    let exposure_time = args[2].parse::<u32>().expect("Cannot parse exposure time!");
    let exposure_gain = args[3].parse::<u16>().expect("Cannot parse exposure gain!");
    let image_path    = &args[4];

    let cam = touptek::Toupcam::open_by_index(camera_index).
                                expect("Need a connected camera!");
    cam.start(|event_rx| {
        cam.set_exposure_time(exposure_time);
        cam.set_exposure_gain(exposure_gain);

        loop {
            match event_rx.recv().unwrap() {
                touptek::Event::Image => {
                    let touptek::Image {
                        resolution: touptek::Resolution { width, height },
                        data, ..
                    } = cam.pull_image(32);

                    let mut file = File::create(image_path).
                                         expect("Cannot open image!");
                    file.write_u32::<BigEndian>(width).unwrap();
                    file.write_u32::<BigEndian>(height).unwrap();
                    file.write(&data).unwrap();

                    break
                },
                _ => ()
            }
        }
    });
}

use crate::emitter::EVENT_EMITTER;
use std::sync::{Arc, Mutex};
extern crate base64;
extern crate image;
extern crate scrap;

use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

static SNAP_WIDTH: u32 = 500;
static SNAP_HEIGHT: u32 = 200;
// Don't capture in Mistlock for now, maybe in the future
// static MISTLOCK_ID: &str = "\"map_id\":872,";
static UNCATEGORIZED_ID: &str = "\"map_id\":947,";
static SNOWBLIND_ID: &str = "\"map_id\":948,";
static SWAMPLAND_ID: &str = "\"map_id\":949,";
static URBAN_ID: &str = "\"map_id\":950,";
static AQUATIC_ID: &str = "\"map_id\":951,";
static CLIFFSIDE_ID: &str = "\"map_id\":952,";
static UNDERGROUND_ID: &str = "\"map_id\":953,";
static VOLCANIC_ID: &str = "\"map_id\":954,";
static MOLTEN_ID: &str = "\"map_id\":955,";
static AETHERBLADE_ID: &str = "\"map_id\":956,";
static THAUMANOVA_ID: &str = "\"map_id\":957,";
static SOLID_ID: &str = "\"map_id\":958,";
static MOLTEN_ID_2: &str = "\"map_id\":959,";
static AETHERBLADE_ID_2: &str = "\"map_id\":960,";
static CHAOS_ID: &str = "\"map_id\":1164,";
static NIGHTMARE_ID: &str = "\"map_id\":1177,";
static SHATTER_ID: &str = "\"map_id\":1205,";
static TWILIGHT_ID: &str = "\"map_id\":1267,";
static DEEPSTONE_ID: &str = "\"map_id\":1290,";
// TODO: Add Sunqua Peak

fn scrap_buffer_to_rgbaimage(w: usize, h: usize, buffer: scrap::Frame) -> image::RgbaImage {
    // Flip the ARGB image into a BGRA image.
    let mut bitflipped = Vec::with_capacity(w * h * 4);
    let stride = buffer.len() / h;
    for y in 0..h {
        for x in 0..w {
            let i = stride * y + 4 * x;
            bitflipped.extend_from_slice(&[buffer[i + 2], buffer[i + 1], buffer[i], 255]);
        }
    }
    image::RgbaImage::from_raw(w as u32, h as u32, bitflipped).unwrap()
}

pub fn setup() {
    thread::spawn(|| {
        let is_first_capture: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let c_is_first_capture: Arc<Mutex<bool>> = Arc::clone(&is_first_capture);
        EVENT_EMITTER
            .lock()
            .unwrap()
            .on("link", move |data: String| {
                if data.contains(UNCATEGORIZED_ID) || data.contains(SNOWBLIND_ID) || data.contains(SWAMPLAND_ID) || data.contains(URBAN_ID) || data.contains(AQUATIC_ID) || data.contains(CLIFFSIDE_ID) || data.contains(UNDERGROUND_ID) || data.contains(VOLCANIC_ID) || data.contains(MOLTEN_ID) || data.contains(AETHERBLADE_ID) || data.contains(THAUMANOVA_ID) || data.contains(SOLID_ID) || data.contains(MOLTEN_ID_2) || data.contains(AETHERBLADE_ID_2) || data.contains(CHAOS_ID) || data.contains(NIGHTMARE_ID) || data.contains(SHATTER_ID) || data.contains(TWILIGHT_ID) || data.contains(DEEPSTONE_ID)
                 {
                    let v = *c_is_first_capture.lock().unwrap();
                    if v == false {
                        *c_is_first_capture.lock().unwrap() = true;

                        let one_second = Duration::new(1, 0);
                        let one_frame = one_second / 60;

                        // Sleep one second to hopefully make it more consistent getting the fractal info and not a loading screen
                        thread::sleep(one_second);

                        let display = Display::primary().expect("Couldn't find primary display.");
                        let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
                        let (w, h) = (capturer.width(), capturer.height());
                        loop {
                            let buffer = match capturer.frame() {
                                Ok(buffer) => buffer,
                                Err(error) => {
                                    if error.kind() == WouldBlock {
                                        // Keep spinning.
                                        thread::sleep(one_frame);
                                        continue;
                                    } else {
                                        panic!("Error: {}", error);
                                    }
                                }
                            };
                            let im = scrap_buffer_to_rgbaimage(w, h, buffer);

                            let cropped = image::imageops::crop_imm(
                                &im,
                                im.width() - SNAP_WIDTH,
                                0,
                                SNAP_WIDTH,
                                SNAP_HEIGHT,
                            );

                            // cropped.to_image().save("cropped.png").unwrap();

                            let di = image::DynamicImage::ImageRgba8(cropped.to_image());

                            let mut buf = Vec::new();
                            di.write_to(&mut buf, image::ImageOutputFormat::Png)
                                .unwrap();
                            let res_base64 = base64::encode(&buf);

                            log::info!("data:image/png;base64,{}", res_base64);
                            break;
                        }
                    }
                } else {
                    *c_is_first_capture.lock().unwrap() = false;
                }
                log::info!(
                    "Is fractal first capture? {} {}",
                    is_first_capture.lock().unwrap(),
                    data
                );
            });
    });
}

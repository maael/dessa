use crate::emitter::EVENT_EMITTER;
use std::sync::{Arc, Mutex};
extern crate base64;
extern crate image;
extern crate scrap;

use chrono::prelude::*;
use scrap::{Capturer, Display};
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

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
static SUNQUA_ID: &str = "\"map_id\":1384,";

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

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OcrResponseMeta {
    fractal_level: u8,
    personal_fractal_level: u8
}

#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct OcrResponse {
    raw: String,
    lines: Vec<String>,
    meta: OcrResponseMeta
}

fn get_ocr (img: String) -> OcrResponse {
    let client = reqwest::blocking::Client::new();
    let mut map = HashMap::new();
    map.insert("img", img);
    log::info!("[ocr] Start");
    let res = client.post("https://dessa.mael.tech/api/ocr")
        .json(&map)
        .send();
    match res {
        Ok(success) => {
            log::info!("[ocr] Success");
            match success.text() {
                Ok(text) => {
                    log::warn!("[ocr] Text Success: {}", text);
                    match serde_json::from_str(&text) {
                        Ok(result) => {
                            log::warn!("[ocr] JSONParse Success");
                            let data: OcrResponse = result;
                            log::info!("data response: {}", data.meta.fractal_level);
                            return data;
                        }, 
                        Err (e) => {
                            log::warn!("[ocr] JSONParse Error: {}", e);
                            return OcrResponse {
                                raw: "".to_string(),
                                lines: vec![],
                                meta: OcrResponseMeta {
                                    fractal_level: 0,
                                    personal_fractal_level: 0
                                }
                            }
                        }
                    }
                },
                Err (e) => {
                    log::warn!("[ocr] Text Error: {}", e);
                    return OcrResponse {
                        raw: "".to_string(),
                        lines: vec![],
                        meta: OcrResponseMeta {
                            fractal_level: 0,
                            personal_fractal_level: 0
                        }
                    }
                }
            }
        },
        Err(e) => {
            log::warn!("[ocr] Error: {}", e);
            return OcrResponse {
                raw: "".to_string(),
                lines: vec![],
                meta: OcrResponseMeta {
                    fractal_level: 0,
                    personal_fractal_level: 0
                }
            }
        }
    }
}

pub fn setup() {
    thread::spawn(|| {
        let is_first_capture: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
        let c_is_first_capture: Arc<Mutex<bool>> = Arc::clone(&is_first_capture);
        EVENT_EMITTER
            .lock()
            .unwrap()
            .on("link", move |data: String| {
                if data.contains(UNCATEGORIZED_ID) || data.contains(SNOWBLIND_ID) || data.contains(SWAMPLAND_ID) || data.contains(URBAN_ID) || data.contains(AQUATIC_ID) || data.contains(CLIFFSIDE_ID) || data.contains(UNDERGROUND_ID) || data.contains(VOLCANIC_ID) || data.contains(MOLTEN_ID) || data.contains(AETHERBLADE_ID) || data.contains(THAUMANOVA_ID) || data.contains(SOLID_ID) || data.contains(MOLTEN_ID_2) || data.contains(AETHERBLADE_ID_2) || data.contains(CHAOS_ID) || data.contains(NIGHTMARE_ID) || data.contains(SHATTER_ID) || data.contains(TWILIGHT_ID) || data.contains(DEEPSTONE_ID) || data.contains(SUNQUA_ID)
                 {
                    let v = *c_is_first_capture.lock().unwrap();
                    if v == false {
                        *c_is_first_capture.lock().unwrap() = true;

                        let one_second = Duration::new(1, 0);
                        let one_frame = one_second / 60;

                        EVENT_EMITTER.lock().unwrap().emit("arc", serde_json::json!({
                            "type": "fractal",
                            "sub_type": "start_fractal",
                            "date_time": format!("{}", Utc::now()),
                        }).to_string());

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

                            let full_base64 = format!("data:image/png;base64,{}", res_base64);

                            log::info!("[ocr] Got image");

                            let result = get_ocr(full_base64);

                            EVENT_EMITTER.lock().unwrap().emit("arc", serde_json::json!({
                                "type": "fractal",
                                "sub_type": "fractal_level",
                                "fractal_level": result.meta.fractal_level,
                                "personal_fractal_level": result.meta.personal_fractal_level
                            }).to_string());
                            
                            break;
                        }
                    }
                } else {
                    let v = *c_is_first_capture.lock().unwrap();
                    if v == true {
                        EVENT_EMITTER.lock().unwrap().emit("arc", serde_json::json!({
                            "type": "fractal",
                            "sub_type": "end_fractal",
                            "date_time": format!("{}", Utc::now()),
                        }).to_string());
                    }
                    *c_is_first_capture.lock().unwrap() = false;
                }
                log::debug!(
                    "Is fractal first capture? {} {}",
                    is_first_capture.lock().unwrap(),
                    data
                );
            });
    });
}

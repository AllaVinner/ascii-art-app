mod utils;

use wasm_bindgen::prelude::*;
use image;

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug, Clone, Copy)]
enum Normalization {
    NONE,
    POW(f64),
    SYMMETRICPOW(f64),
    DOUBLEPOW(f64, f64),
}

#[derive(Debug, Clone, Copy)]
enum Lighting {
    DARK,
    LIGHT
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(a: i32) {
    console_log!("Hello {}!", a);
}

#[wasm_bindgen]
pub fn add1(v: Vec<i32>) {
    let v2: Vec<i32> = v.iter().map(|i| i+1).collect();
    for i in 1..(v2.len()) {
        console_log!("Hello {}!", v2[i]);
    }
    
}


#[wasm_bindgen(js_name=printImage)]
pub fn print_image(data: Vec<u8>) {
    console_log!("Length from Rust is: {}", data.len());
    let im = image::load_from_memory(&data).unwrap().into_luma8();
    console_log!("Width from Rust is: {}", im.width());
    console_log!("Height from Rust is: {}", im.height());
    
}


#[wasm_bindgen(js_name=convertImage)]
pub fn convert_image(data: Vec<u8>, num_ascii_rows: u32) -> String{
    let im: ImageBuffer<Luma<u8>, Vec<u8>> = image::load_from_memory(&data).unwrap().into_luma8();
    let characters = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~';0,".chars().collect();
    let coverage = vec![0.2911208734877755, 0.3087129931486686, 0.8014105586245723, 0.8749263943017404, 0.7801559113184721, 0.7854330600819132, 0.15435654781418623, 0.4296115719332253, 0.4296121883800508, 0.4056937299816192, 0.3248813568360632, 0.19124604221633126, 0.176228194877868, 0.11711997305782858, 0.431335338035663, 0.6578362852362656, 0.4784232953355356, 0.6319377764007731, 0.5919144018742041, 0.6473659825426034, 0.6179391702378915, 0.6750618787924267, 0.4598147092539898, 0.7368787412749952, 0.6750619611143555, 0.2342401346296277, 0.30836614035783283, 0.38240194710877357, 0.35245656743888176, 0.3824010657043953, 0.42717962116733366, 0.9022066374895383, 0.7878029298843567, 0.8512611124366641, 0.6319997082044945, 0.766941868696177, 0.869159673799146, 0.7350856824365698, 0.7731849854564686, 0.8517882613870955, 0.5246381110853061, 0.585590980011913, 0.8383615774173173, 0.6285664942998616, 1.0, 0.900326385914531, 0.7256445614868275, 0.74047838672887, 0.9319414625473496, 0.8212606706145796, 0.7733951139252226, 0.713006013068589, 0.7199902284336999, 0.6565197891860448, 0.931814682606195, 0.8162673751036729, 0.6573644099105355, 0.7344440706845351, 0.5427738476810492, 0.4313253387703526, 0.5427739794317149, 0.294832562646954, 0.25906797183729197, 0.11875441089769623, 0.683341267963664, 0.7823045222122592, 0.5528767906498866, 0.7944692098499746, 0.6693841202516828, 0.6476514953162877, 0.8715377316230339, 0.7429897212631813, 0.48669789827874577, 0.5677615375623823, 0.7934986151573116, 0.5089404577549033, 0.8459026783478254, 0.6562528804284758, 0.5865580165532801, 0.8289968751573783, 0.828996924009571, 0.5132225227249239, 0.6708815616098251, 0.5245217619856687, 0.6034444110061499, 0.5463902516406081, 0.7524725625737766, 0.6968935461904168, 0.6584080165944618, 0.6344727373896233, 0.5148927510505888, 0.3867917707211087, 0.5148928894236053, 0.222631847913224, 0.15435654781418623, 0.30836614035783283];
    let height_width_ratio = 1.8729642629623413;
    to_ascii(im, characters, coverage, height_width_ratio, num_ascii_rows)
}



///////////////////////////////
/// 
/// 
/// 

use image::{GenericImageView, GrayImage, ImageBuffer, Luma};


fn find_closest_match(pixel: f64, characters: &Vec<char>, coverage: &Vec<f64>) -> char {
    let mut best_value: f64 = 10000000.;
    let mut best_char = '!';
    for (c, v) in characters.iter().zip(coverage.iter()) {
        if (*v-pixel)*(*v-pixel) < best_value {
            best_value = (*v-pixel)*(*v-pixel);
            best_char = *c;
        }
    }
    best_char
}

pub fn get_value_image_from_rows(img: ImageBuffer<Luma<u8>, Vec<u8>>, num_rows: u32, font_scale: f64) -> GrayImage {

    let image_height = img.height() as f64;
    let image_width = img.width() as f64;

    let window_height = (image_height/num_rows as f64).ceil();

    let window_width = (window_height/font_scale).floor() as f64;
    let num_columns = (image_width / window_width).floor() as f64;

    let mut value_image: GrayImage = ImageBuffer::new(num_columns as u32, num_rows as u32);
    for (rowi, row) in (0..(img.height()-window_height as u32)).step_by(window_height as usize).enumerate() {       
        for (coli, col) in (0..(img.width()-window_width as u32)).step_by(window_width as usize).enumerate() {
            let sub_view: image::SubImage<&image::ImageBuffer<image::Luma<u8>, Vec<u8>>> = img.view(col, row, window_width as u32, window_height as u32);
            let mut window_value = 0.;
            for p in sub_view.pixels() {
                window_value += p.2.0[0] as f64;
            }           
            let p = value_image.get_pixel_mut(coli as u32, rowi as u32);
            *p = Luma([((window_value / (window_height*window_width)).floor()) as u8]);
        }

    }
    return value_image;
}

fn to_ascii(img: ImageBuffer<Luma<u8>, Vec<u8>>, characters: Vec<char>, coverage: Vec<f64>, height_width_ratio: f64, num_ascii_rows: u32) -> String {
    //let ascii_num_rows = 20;
    let lighting = Lighting::DARK;
    let normalization = Normalization::NONE;

    //let height_width_ratio = 1.8729642629623413;
    let value_image = get_value_image_from_rows(img, num_ascii_rows, height_width_ratio);
    let mut s: String = "".to_string();
    for (i, p) in value_image.pixels().enumerate() {
        if i as u32 % value_image.width() == 0{
            if i != 0 {
                s.push('\n');
            }
        }
        let mut val = p.0[0] as f64 / 255.;
        val = match lighting {
            Lighting::DARK => val,
            Lighting::LIGHT => 1. - val,
        };
        val = match normalization {
            Normalization::NONE => val,
            Normalization::POW(p) => val.powf(p),
            Normalization::SYMMETRICPOW(p) => (val.powf(p) + val.powf(1./p))/2.,
            Normalization::DOUBLEPOW(tail, head) => (val.powf(1./tail) + val.powf(head))/2., 
        };
        s.push(find_closest_match(val, &characters, &coverage))
        
    }
    return s;
}




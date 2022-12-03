// Translated from the example in olive-c/README.md

use std::mem;
use std::ffi::CString;
use olivec_sys::*;
use stb_sys::stbi_write_png;

const CANVAS_WIDTH: u64 = 900;
const CANVAS_HEIGHT: u64 = 600;

fn main() {
    let mut pixels = [0u32; (CANVAS_WIDTH * CANVAS_HEIGHT) as usize];

    let canvas = unsafe { olivec_canvas(pixels.as_mut_ptr(), CANVAS_WIDTH, CANVAS_HEIGHT, CANVAS_WIDTH) };
    // Taken from https://upload.wikimedia.org/wikipedia/en/9/9e/Flag_of_Japan.svg
    unsafe {
        olivec_fill(canvas, 0xFFFFFFFF);
        olivec_circle(canvas, (CANVAS_WIDTH/2) as i32, (CANVAS_HEIGHT/2) as i32, 180, 0xFF2D00BC);
    }

    let file_path = "flag_jp.png";
    let c_file_path = CString::new(file_path).expect("couldn't create CString");

    let res = unsafe {
        stbi_write_png(c_file_path.as_ptr(), CANVAS_WIDTH as i32, CANVAS_HEIGHT as i32, 4, mem::transmute(pixels.as_slice().as_ptr()), (mem::size_of::<u32>()*CANVAS_WIDTH as usize) as i32)
    };
    if res == 0 {
        eprintln!("ERROR: could not write {}\n", file_path);
        std::process::exit(1);
    }
}

#![feature(test)]

extern crate test;
use rgb::FromSlice;
use test::Bencher;

use resize::Pixel::{Gray16, Gray8, RGB8, RGBA16, RGBA16P};
use resize::Type::Triangle;
use resize::Type::Lanczos3;
use resize::Type::Point;
use std::fs::File;
use std::path::PathBuf;

fn get_image() -> (png::OutputInfo, Vec<u8>) {
    let root: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    let decoder = png::Decoder::new(File::open(root.join("examples/tiger.png")).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut src = vec![110; info.buffer_size()];
    reader.next_frame(&mut src).unwrap();
    (info, src)
}

#[bench]
fn precomputed_large(b: &mut Bencher) {
    let (info, src) = get_image();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (1600, 1200);
    let mut dst = vec![120; w2 * h2];

    let mut r = resize::new(w1, h1, w2, h2, Gray8, Triangle).unwrap();

    b.iter(|| r.resize(src.as_gray(), dst.as_gray_mut()).unwrap());
}

#[bench]
fn tiny(b: &mut Bencher) {
    let (info, src0) = get_image();
    let (w0, h0) = (info.width as usize, info.height as usize);
    let (w1, h1) = (3200, 2400);
    let mut src1 = vec![0; w1 * h1];

    resize::new(w0, h0, w1, h1, Gray8, Triangle).unwrap().resize(src0.as_gray(), src1.as_gray_mut()).unwrap();
    let (w2, h2) = (16, 12);
    let mut dst = vec![0; w2 * h2];

    let mut r = resize::new(w1, h1, w2, h2, Gray8, Lanczos3).unwrap();
    b.iter(|| r.resize(src1.as_gray(), dst.as_gray_mut()).unwrap());
}

#[bench]
fn huge_stretch(b: &mut Bencher) {
    let (info, src0) = get_image();
    let (w0, h0) = (info.width as usize, info.height as usize);
    let (w1, h1) = (12, 12);
    let mut src1 = vec![0; w1 * h1];

    resize::new(w0, h0, w1, h1, Gray8, Lanczos3).unwrap().resize(src0.as_gray(), src1.as_gray_mut()).unwrap();
    let (w2, h2) = (1200, 1200);
    let mut dst = vec![0; w2 * h2];

    let mut r = resize::new(w1, h1, w2, h2, Gray8, Triangle).unwrap();
    b.iter(|| r.resize(src1.as_gray(), dst.as_gray_mut()).unwrap());
}

#[bench]
fn precomputed_small(b: &mut Bencher) {
    let (info, src) = get_image();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100, 100);
    let mut dst = vec![240; w2 * h2];

    let mut r = resize::new(w1, h1, w2, h2, Gray8, Triangle).unwrap();

    b.iter(|| r.resize(src.as_gray(), dst.as_gray_mut()).unwrap());
}

#[bench]
fn a_small_rgb(b: &mut Bencher) {
    let (info, src) = get_image();
    let src: Vec<_> = src.into_iter().map(|c| rgb::RGB::new(c,c,c)).collect();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100, 100);
    let mut dst = vec![240; w2 * h2 * 3];

    let mut r = resize::new(w1, h1, w2, h2, RGB8, Triangle).unwrap();

    b.iter(|| r.resize(&src, dst.as_rgb_mut()).unwrap());
}

#[bench]
fn a_small_rgba16(b: &mut Bencher) {
    let (info, src) = get_image();
    let src: Vec<_> = src.into_iter().map(|c| {
        let w = ((c as u16) << 8) | c as u16;
        rgb::RGBA::new(w,w,w,65535)
    }).collect();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100, 100);
    let mut dst = vec![0u16; w2 * h2 * 4];

    let mut r = resize::new(w1, h1, w2, h2, RGBA16, Triangle).unwrap();

    b.iter(|| r.resize(&src, dst.as_rgba_mut()).unwrap());
}

#[bench]
fn a_small_rgba16_premultiplied(b: &mut Bencher) {
    let (info, src) = get_image();
    let src: Vec<_> = src.into_iter().map(|c| {
        let w = ((c as u16) << 8) | c as u16;
        rgb::RGBA::new(w,w,w,65535)
    }).collect();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100, 100);
    let mut dst = vec![0u16; w2 * h2 * 4];

    let mut r = resize::new(w1, h1, w2, h2, RGBA16P, Triangle).unwrap();

    b.iter(|| r.resize(&src, dst.as_rgba_mut()).unwrap());
}

#[bench]
fn precomputed_small_16bit(b: &mut Bencher) {
    let (info, src) = get_image();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100,100);
    let mut dst = vec![33; w2*h2];
    let src: Vec<_> = src.into_iter().map(|px|{
        let px = px as u16;
        (px << 8) | px
    }).collect();

    let mut r = resize::new(w1, h1, w2, h2, Gray16, Triangle).unwrap();

    b.iter(|| r.resize(src.as_gray(), dst.as_gray_mut()).unwrap());
}

#[bench]
#[allow(deprecated)]
fn recomputed_small(b: &mut Bencher) {
    let (info, src) = get_image();
    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100, 100);
    let mut dst = vec![99; w2 * h2];

    b.iter(|| resize::resize(w1, h1, w2, h2, Gray8, Triangle, src.as_gray(), dst.as_gray_mut()).unwrap());
}

#[bench]
#[allow(deprecated)]
fn init_lanczos(b: &mut Bencher) {
    b.iter(|| resize::new(test::black_box(100), 200, test::black_box(300), 400, RGB8, Lanczos3).unwrap());
}

#[bench]
#[allow(deprecated)]
fn init_point(b: &mut Bencher) {
    b.iter(|| resize::new(test::black_box(100), 200, test::black_box(300), 400, RGB8, Point).unwrap());
}

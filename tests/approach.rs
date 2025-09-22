use kenzu::Builder;
use malakoi::Math;

#[derive(Debug, Builder, Math, Clone, Default)]
pub struct CalcStruct {
    pub usize: usize,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub u128: u128,
    pub isize: isize,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub i128: i128,
    pub f64: f64,
    pub f32: f32,
}
#[test]
fn approach_usize() {
    let mut calc_struct = CalcStruct::new().usize::<usize>(10).build().unwrap();
    calc_struct.approach_usize(20, 6);
    assert_eq!(calc_struct.usize, 16);
    calc_struct.approach_usize(20, 6);
    assert_eq!(calc_struct.usize, 20);
}

#[test]
fn approach_u8() {
    let mut calc_struct = CalcStruct::new().u8::<u8>(10).build().unwrap();
    calc_struct.approach_u8(0, 5);
    assert_eq!(calc_struct.u8, 5);
    calc_struct.approach_u8(0, 5);
    assert_eq!(calc_struct.u8, 0);
}

#[test]
fn approach_u16() {
    let mut calc_struct = CalcStruct::new().u16::<u16>(30).build().unwrap();
    calc_struct.approach_u16(50, 15);
    assert_eq!(calc_struct.u16, 45);
    calc_struct.approach_u16(50, 15);
    assert_eq!(calc_struct.u16, 50);
}

#[test]
fn approach_u32() {
    let mut calc_struct = CalcStruct::new().u32::<u32>(40).build().unwrap();
    calc_struct.approach_u32(30, 5);
    assert_eq!(calc_struct.u32, 35);
    calc_struct.approach_u32(30, 5);
    assert_eq!(calc_struct.u32, 30);
}

#[test]
fn approach_u64() {
    let mut calc_struct = CalcStruct::new().u64::<u64>(50).build().unwrap();
    calc_struct.approach_u64(100, 30);
    assert_eq!(calc_struct.u64, 80);
    calc_struct.approach_u64(100, 30);
    assert_eq!(calc_struct.u64, 100);
}

#[test]
fn approach_u128() {
    let mut calc_struct = CalcStruct::new().u128::<u128>(60).build().unwrap();
    calc_struct.approach_u128(90, 20);
    assert_eq!(calc_struct.u128, 80);
    calc_struct.approach_u128(90, 20);
    assert_eq!(calc_struct.u128, 90);
}

#[test]
fn approach_isize() {
    let mut calc_struct = CalcStruct::new().isize::<isize>(70).build().unwrap();
    calc_struct.approach_isize(50, 15);
    assert_eq!(calc_struct.isize, 55);
    calc_struct.approach_isize(50, 15);
    assert_eq!(calc_struct.isize, 50);
}

#[test]
fn approach_i16() {
    let mut calc_struct = CalcStruct::new().i16::<i16>(80).build().unwrap();
    calc_struct.approach_i16(100, 10);
    assert_eq!(calc_struct.i16, 90);
    calc_struct.approach_i16(100, 10);
    assert_eq!(calc_struct.i16, 100);
}

#[test]
fn approach_i32() {
    let mut calc_struct = CalcStruct::new().i32(90).build().unwrap();
    calc_struct.approach_i32(50, 25);
    assert_eq!(calc_struct.i32, 65);
    calc_struct.approach_i32(50, 25);
    assert_eq!(calc_struct.i32, 50);
}

#[test]
fn approach_i64() {
    let mut calc_struct = CalcStruct::new().i64(100).build().unwrap();
    calc_struct.approach_i64(120, 15);
    assert_eq!(calc_struct.i64, 115);
    calc_struct.approach_i64(120, 15);
    assert_eq!(calc_struct.i64, 120);
}

#[test]
fn approach_i128() {
    let mut calc_struct = CalcStruct::new().i128(110).build().unwrap();
    calc_struct.approach_i128(100, 8);
    assert_eq!(calc_struct.i128, 102);
    calc_struct.approach_i128(100, 8);
    assert_eq!(calc_struct.i128, 100);
}

#[test]
fn approach_f64() {
    let mut calc_struct = CalcStruct::new().f64(120.0).build().unwrap();
    calc_struct.approach_f64(130.0, 5.0);
    assert_eq!(calc_struct.f64, 125.0);
    calc_struct.approach_f64(130.0, 5.0);
    assert_eq!(calc_struct.f64, 130.0);
}

#[test]
fn approach_f32() {
    let mut calc_struct = CalcStruct::new().f32(130.0).build().unwrap();
    calc_struct.approach_f32(120.0, 8.0);
    assert_eq!(calc_struct.f32, 122.0);
    calc_struct.approach_f32(120.0, 8.0);
    assert_eq!(calc_struct.f32, 120.0);
}

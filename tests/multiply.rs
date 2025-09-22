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
fn mul_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_usize(5);
    assert_eq!(calc_struct.usize, 5);
}

#[test]
fn mul_u8() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_u8(5);
    assert_eq!(calc_struct.u8, 10);
}

#[test]
fn mul_u16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_u16(5);
    assert_eq!(calc_struct.u16, 15);
}

#[test]
fn mul_u32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_u32(5);
    assert_eq!(calc_struct.u32, 20);
}

#[test]
fn mul_u64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_u64(5);
    assert_eq!(calc_struct.u64, 25);
}

#[test]
fn mul_u128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_u128(5);
    assert_eq!(calc_struct.u128, 30);
}

#[test]
fn mul_isize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_isize(5);
    assert_eq!(calc_struct.isize, 35);
}

#[test]
fn mul_i16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_i16(5);
    assert_eq!(calc_struct.i16, 40);
}

#[test]
fn mul_i32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_i32(5);
    assert_eq!(calc_struct.i32, 45);
}

#[test]
fn mul_i64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_i64(5);
    assert_eq!(calc_struct.i64, 50);
}

#[test]
fn mul_i128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_i128(5);
    assert_eq!(calc_struct.i128, 55);
}

#[test]
fn mul_f64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_f64(5.0);
    assert_eq!(calc_struct.f64, 60.0);
}

#[test]
fn mul_f32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .u8::<u8>(2)
        .u16::<u16>(3)
        .u32::<u32>(4)
        .u64::<u64>(5)
        .u128::<u128>(6)
        .isize::<isize>(7)
        .i16::<i16>(8)
        .i32(9)
        .i64(10)
        .i128(11)
        .f64(12.0)
        .f32(13.0)
        .build()
        .unwrap();
    calc_struct.mul_f32(5.0);
    assert_eq!(calc_struct.f32, 65.0);
}

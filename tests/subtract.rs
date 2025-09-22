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
fn sub_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_usize(5);

    assert_eq!(calc_struct.usize, 1);
}

#[test]
fn sub_u8() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_u8(5);
    assert_eq!(calc_struct.u8, 2);
}

#[test]
fn sub_u16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_u16(5);
    assert_eq!(calc_struct.u16, 3);
}

#[test]
fn sub_u32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_u32(5);
    assert_eq!(calc_struct.u32, 4);
}

#[test]
fn sub_u64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_u64(5);
    assert_eq!(calc_struct.u64, 5);
}

#[test]
fn sub_u128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_u128(5);
    assert_eq!(calc_struct.u128, 6);
}

#[test]
fn sub_isize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_isize(5);

    assert_eq!(calc_struct.isize, 7);
}

#[test]
fn sub_i16() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_i16(5);
    assert_eq!(calc_struct.i16, 8);
}

#[test]
fn sub_i32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_i32(5);
    assert_eq!(calc_struct.i32, 9);
}

#[test]
fn sub_i64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_i64(5);
    assert_eq!(calc_struct.i64, 10);
}

#[test]
fn sub_i128() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_i128(5);
    assert_eq!(calc_struct.i128, 11);
}

#[test]
fn sub_f64() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_f64(5.0);
    assert_eq!(calc_struct.f64, 12.0);
}

#[test]
fn sub_f32() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .u8::<u8>(7)
        .u16::<u16>(8)
        .u32::<u32>(9)
        .u64::<u64>(10)
        .u128::<u128>(11)
        .isize::<isize>(12)
        .i16::<i16>(13)
        .i32(14)
        .i64(15)
        .i128(16)
        .f64(17.0)
        .f32(18.0)
        .build()
        .unwrap();
    calc_struct.sub_f32(5.0);
    assert_eq!(calc_struct.f32, 13.0);
}

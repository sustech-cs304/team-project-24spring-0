use crate::assembler::rv32f::RV32F;
use crate::assembler::rv32i::RV32I;

#[test]
fn test_lui() {
    assert_eq!(
        0x000231b7,
        Into::<u32>::into(RV32I::lui(0x23.into(), 3.into()))
    );
}

#[test]
fn test_jal() {
    assert_eq!(
        0x0dc0006f,
        Into::<u32>::into(RV32I::jal(0xDC.into(), 0x0.into()))
    );
}

#[test]
fn test_blt() {
    assert_eq!(
        0xf4f740e3,
        Into::<u32>::into(RV32I::blt(0xF40.into(), 0xF.into(), 0xE.into()))
    );
}

#[test]
fn test_lb() {
    assert_eq!(
        0b10101010101000010000000110000011,
        Into::<u32>::into(RV32I::lb(0xAAA.into(), 0x2.into(), 0x3.into()))
    );
}

#[test]
fn test_sb() {
    assert_eq!(
        0xfca42e23,
        Into::<u32>::into(RV32I::sw(0xFDC.into(), 0xA.into(), 0x8.into()))
    )
}

#[test]
fn test_addi() {
    assert_eq!(
        0b01111010101000010000000110010011,
        Into::<u32>::into(RV32I::addi(0x7AA.into(), 0x2.into(), 0x3.into()))
    );
}

#[test]
fn test_slli() {
    assert_eq!(
        0b101000010001000110010011,
        Into::<u32>::into(RV32I::slli(0xA.into(), 0x2.into(), 0x3.into()))
    );
}

#[test]
fn test_add() {
    assert_eq!(
        0b101000010000000110110011,
        Into::<u32>::into(RV32I::add(0xA.into(), 0x2.into(), 0x3.into()))
    );
}

#[test]
fn test_csrrw() {
    assert_eq!(
        0b00110000010001100001011011110011,
        Into::<u32>::into(RV32I::csrrw(0x304.into(), 0xC.into(), 0xD.into()))
    )
}

#[test]
fn test_flw() {
    assert_eq!(
        0x00042407,
        Into::<u32>::into(RV32F::flw(0x0.into(), 0x8.into(), 0x8.into()))
    )
}

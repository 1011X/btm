use registers;
use registers::{Register, StandardRegister, SystemRegister};

#[test]
fn standard_register_from_trit4() {
    assert_eq!(
        Ok(registers::ZERO),
        StandardRegister::from_trit4(0b00_00_00_00)
    );
    assert_eq!(
        Ok(registers::LO),
        StandardRegister::from_trit4(0b00_00_00_01)
    );
    assert_eq!(
        Ok(registers::HI),
        StandardRegister::from_trit4(0b00_00_01_11)
    );
    assert_eq!(
        Ok(registers::SP),
        StandardRegister::from_trit4(0b00_00_01_00)
    );
    assert_eq!(
        Ok(registers::FP),
        StandardRegister::from_trit4(0b00_00_01_01)
    );
    assert_eq!(
        Ok(registers::RA),
        StandardRegister::from_trit4(0b00_01_11_11)
    );
    assert_eq!(
        Ok(registers::A0),
        StandardRegister::from_trit4(0b00_01_11_00)
    );
    assert_eq!(
        Ok(registers::A1),
        StandardRegister::from_trit4(0b00_01_11_01)
    );
    assert_eq!(
        Ok(registers::A2),
        StandardRegister::from_trit4(0b00_01_00_11)
    );
    assert_eq!(
        Ok(registers::A3),
        StandardRegister::from_trit4(0b00_01_00_00)
    );
    assert_eq!(
        Ok(registers::A4),
        StandardRegister::from_trit4(0b00_01_00_01)
    );
    assert_eq!(
        Ok(registers::A5),
        StandardRegister::from_trit4(0b00_01_01_11)
    );
    assert_eq!(
        Ok(registers::S0),
        StandardRegister::from_trit4(0b00_01_01_00)
    );
    assert_eq!(
        Ok(registers::S1),
        StandardRegister::from_trit4(0b00_01_01_01)
    );
    assert_eq!(
        Ok(registers::S2),
        StandardRegister::from_trit4(0b01_11_11_11)
    );
    assert_eq!(
        Ok(registers::S3),
        StandardRegister::from_trit4(0b01_11_11_00)
    );
    assert_eq!(
        Ok(registers::S4),
        StandardRegister::from_trit4(0b01_11_11_01)
    );
    assert_eq!(
        Ok(registers::S5),
        StandardRegister::from_trit4(0b01_11_00_11)
    );
    assert_eq!(
        Ok(registers::T0),
        StandardRegister::from_trit4(0b01_11_00_00)
    );
    assert_eq!(
        Ok(registers::T1),
        StandardRegister::from_trit4(0b01_11_00_01)
    );
    assert_eq!(
        Ok(registers::T2),
        StandardRegister::from_trit4(0b01_11_01_11)
    );
    assert_eq!(
        Ok(registers::T3),
        StandardRegister::from_trit4(0b01_11_01_00)
    );
    assert_eq!(
        Ok(registers::T4),
        StandardRegister::from_trit4(0b01_11_01_01)
    );
    assert_eq!(
        Ok(registers::T5),
        StandardRegister::from_trit4(0b01_00_11_11)
    );

    assert!(StandardRegister::from_trit4(0b00_00_00_11).is_err());
    assert!(StandardRegister::from_trit4(0b01_00_11_00).is_err());
}

#[test]
fn system_register_from_trit4() {
    assert_eq!(
        Ok(registers::EHA),
        SystemRegister::from_trit4(0b00_00_00_00)
    );
    assert_eq!(
        Ok(registers::ERA),
        SystemRegister::from_trit4(0b00_00_00_01)
    );
    assert_eq!(Ok(registers::EC), SystemRegister::from_trit4(0b00_00_01_11));
    assert_eq!(Ok(registers::ED), SystemRegister::from_trit4(0b00_00_01_00));

    assert!(SystemRegister::from_trit4(0b00_00_00_11).is_err());
    assert!(SystemRegister::from_trit4(0b00_00_01_01).is_err());
}

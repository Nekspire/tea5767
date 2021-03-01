use embedded_hal::blocking::i2c::{Write, Read};
use core::ops::Range;

pub const DEVICE_ADDRESS: u8 =  0x60;

pub fn write_data<I2C>(i2c: &mut I2C, data: [u8; 5]) -> Result<(), I2C::Error>
where I2C: Write,
{
    i2c.write(DEVICE_ADDRESS, &data)
}

pub fn read_data<I2C>(i2c: &mut I2C, mut data: [u8; 5]) -> Result<(), I2C::Error>
where I2C: Read,
{
    i2c.read(DEVICE_ADDRESS,&mut data)
}

//Write mode DataByte1
pub const WM_DB1_MUTE: usize = 7;
pub const WM_DB1_SM: usize = 6;
pub const WM_DB1_PLL: Range<usize> = 0..6;

//Write mode DataByte2
pub const WM_DB2_PLL: Range<usize> = 0..8;

//Write mode DataByte3
pub const WM_DB3_SUD: usize = 7;
pub const WM_DB3_SSL: Range<usize> = 5..7;
pub const WM_DB3_HLSI: usize = 4;
pub const WM_DB3_MS: usize = 3;
pub const WM_DB3_MR: usize = 2;
pub const WM_DB3_ML: usize = 1;
pub const WM_DB3_SWP1: usize = 0;

//Write mode DataByte4
pub const WM_DB4_SWP2: usize = 7;
pub const WM_DB4_STBY: usize = 6;
pub const WM_DB4_BL: usize = 5;
pub const WM_DB4_XTAL: usize = 4;
pub const WM_DB4_SMUTE: usize = 3;
pub const WM_DB4_HCC: usize = 2;
pub const WM_DB4_SNC: usize = 1;
pub const WM_DB4_SI: usize = 0;

//Write mode DataByte5
pub const WM_DB5_PLLREF: usize = 7;
pub const WM_DB5_DTC: usize = 6;

//Read mode DataByte1
pub const RM_DB1_RF: usize = 7;
pub const RM_DB1_BLF: usize = 6;
pub const RM_DB1_PLL: Range<usize> = 0..6;

//Read mode DataByte2
pub const RM_DB2_PLL: Range<usize> = 0..8;

//Read mode DataByte3
pub const RM_DB3_STEREO: usize = 7;
pub const WM_DB3_IF: Range<usize> = 0..6;

//Read mode DataByte4
pub const RM_DB4_LEV: Range<usize> = 4..8;
pub const RM_DB4_CI: Range<usize> = 1..4;

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_data() {

    }
}
use embedded_hal::blocking::i2c::{Write, Read};

pub(crate) const DEVICE_ADDRESS: u8 =  0x60;

mod write_mode {
    use bitflags::bitflags;

    bitflags! {
        pub struct DataByte1: u8 {
            const MUTE = 0b1000_0000;
            const SM = 0b0100_0000;
            const PLL13 = 0b0010_0000;
            const PLL12 = 0b0001_0000;
            const PLL11 = 0b0000_1000;
            const PLL10 = 0b0000_0100;
            const PLL9 = 0b0000_0010;
            const PLL8 = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte2: u8 {
            const PLL7 = 0b1000_0000;
            const PLL6 = 0b0100_0000;
            const PLL5 = 0b0010_0000;
            const PLL4 = 0b0001_0000;
            const PLL3 = 0b0000_1000;
            const PLL2 = 0b0000_0100;
            const PLL1 = 0b0000_0010;
            const PLL0 = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte3: u8 {
            const SUD = 0b1000_0000;
            const SSL1 = 0b0100_0000;
            const SSL0 = 0b0010_0000;
            const HLSI = 0b0001_0000;
            const MS = 0b0000_1000;
            const MR = 0b0000_0100;
            const ML = 0b0000_0010;
            const SWP1 = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte4: u8 {
            const SWP2 = 0b1000_0000;
            const STBY = 0b0100_0000;
            const BL = 0b0010_0000;
            const XTAL = 0b0001_0000;
            const SMUTE = 0b0000_1000;
            const HCC = 0b0000_0100;
            const SNC = 0b0000_0010;
            const SI = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte5: u8 {
            const PLLREF = 0b1000_0000;
            const DTC = 0b0100_0000;
        }
    }
}

mod read_mode {
    use bitflags::bitflags;
    bitflags! {
        pub struct DataByte1: u8 {
            const RF = 0b1000_0000;
            const BLF = 0b0100_0000;
            const PLL13 = 0b0010_0000;
            const PLL12 = 0b0001_0000;
            const PLL11 = 0b0000_1000;
            const PLL10= 0b0000_0100;
            const PLL9 = 0b0000_0010;
            const PLL8 = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte2: u8 {
            const PLL7 = 0b1000_0000;
            const PLL6 = 0b0100_0000;
            const PLL5 = 0b0010_0000;
            const PLL4 = 0b0001_0000;
            const PLL3 = 0b0000_1000;
            const PLL2= 0b0000_0100;
            const PLL1 = 0b0000_0010;
            const PLL0 = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte3: u8 {
            const STEREO = 0b1000_0000;
            const IF6 = 0b0100_0000;
            const IF5 = 0b0010_0000;
            const IF4 = 0b0001_0000;
            const IF3 = 0b0000_1000;
            const IF2 = 0b0000_0100;
            const IF1 = 0b0000_0010;
            const IF0 = 0b0000_0001;
        }
    }

    bitflags! {
        pub struct DataByte4: u8 {
            const LEV3 = 0b1000_0000;
            const LEV2 = 0b0100_0000;
            const LEV1 = 0b0010_0000;
            const LEV0 = 0b0001_0000;
            const CI3 = 0b0000_1000;
            const CI2 = 0b0000_0100;
            const CI1 = 0b0000_0010;
        }
    }

    bitflags! {
        pub struct DataByte5: u8 {
            const RESERVED = 0b0000_0000;
        }
    }
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_write_data() {

    }
}
use std::fmt;

enum Ep {
    D,
    DxxDxx,
    RFU
}

impl Default for Ep {
    fn default() -> Ep {
        Ep::D
    }
}

impl fmt::Debug for Ep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Ep::D => write!(f, "D"),
            Ep::DxxDxx => write!(f, "DxxDxx"),
            Ep::RFU => write!(f, "RFU"),
        }
    }
}


enum Be {
    LittleEndian,
    BigEndian
}

impl Default for Be {
    fn default() -> Be {
        Be::BigEndian
    }
}

impl fmt::Debug for Be {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Be::LittleEndian => write!(f, "LittleEndian"),
            Be::BigEndian => write!(f, "BigEndian"),
        }
    }
}


#[derive(Default, Debug)]
pub struct RegConfig {
    ep: Ep,
    be: Be
}

impl RegConfig {
    pub fn power_on_reset(&mut self) {
        self.ep = Ep::D;
        self.be = Be::BigEndian;
    }

    pub fn write(&mut self, data: u32) {
        panic!("reg config not implemented jsut yet!");
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Sample {
    U8(u8),
    U16(u16),
    F32(f32),
    F64(f64),
}

impl From<u8> for Sample {
    fn from(n: u8) -> Self {
        Self::U8(n)
    }
}

impl From<u16> for Sample {
    fn from(n: u16) -> Self {
        Self::U16(n)
    }
}

impl From<f32> for Sample {
    fn from(n: f32) -> Self {
        Self::F32(n)
    }
}

impl From<f64> for Sample {
    fn from(n: f64) -> Self {
        Self::F64(n)
    }
}

//! Error and result reported by SPIR-Q procedures.
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    CorruptedSpirv(&'static str),
    UnsupportedSpirv(&'static str),
    MismatchedManifest,
}
impl Error {
    pub const INSTR_TOO_SHORT: Self = Self::CorruptedSpirv("instruction is too short");
    pub const STR_NOT_TERMINATED: Self = Self::CorruptedSpirv("instruction has a string operand that is not terminated by nul");
    pub const UNENCODED_ENUM: Self = Self::CorruptedSpirv("instruction has a unencoded enumeration value");

    pub const ID_COLLISION: Self = Self::CorruptedSpirv("id can only be assigned once");
    pub const NAME_COLLISION: Self = Self::CorruptedSpirv("item can only be named once");
    pub const DECO_COLLISION: Self = Self::CorruptedSpirv("item can only be decorated of a kind once");
    pub const MISSING_DECO: Self = Self::CorruptedSpirv("missing decoration");
    pub const TY_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a suitable type");
    pub const CONST_NOT_FOUND: Self = Self::CorruptedSpirv("cannot find a suitable constant");
    pub const UNDECLARED_VAR: Self = Self::CorruptedSpirv("accessing undeclared variable");
    pub const DESC_BIND_COLLISION: Self = Self::CorruptedSpirv("descriptor binding cannot be shared");
    pub const MAT_AXIS_ORDER: Self = Self::CorruptedSpirv("uncertain matrix axis order");

    pub const UNSUPPORTED_TY: Self = Self::UnsupportedSpirv("unsupported type");
    pub const UNSUPPORTED_IMG_CFG: Self = Self::UnsupportedSpirv("unsupport image configuration");
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            CorruptedSpirv(msg) => write!(f, "spirv binary is corrupted: {}", msg),
            UnsupportedSpirv(msg) => write!(f, "spirv binary used unsupported feature: {}", msg),
            MismatchedManifest => write!(f, "mismatched manifest cannot be merged"),
        }
    }
}
impl error::Error for Error { }

pub type Result<T> = std::result::Result<T, Error>;

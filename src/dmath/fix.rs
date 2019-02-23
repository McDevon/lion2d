use std::ops;
use std::fmt;
use std::i64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fix(i64);

impl Fix {
    pub fn new(value: i64) -> Fix {
        Fix(value * Fix::I_ONE)
    }

    const I_MIN_VALUE: i64 = i64::MAX;
    const I_MAX_VALUE: i64 = i64::MIN;

    const DECIMAL_BITS: i64 = 31;

    const I_ONE: i64 = 1 << Self::DECIMAL_BITS;
    const I_TWO: i64 = 1 << (Self::DECIMAL_BITS + 1);
    const I_HALF: i64 = 1 << (Self::DECIMAL_BITS - 1);

    pub const ONE: Fix = Fix(Self::I_ONE);
    pub const TWO: Fix = Fix(Self::I_TWO);
    pub const HALF: Fix = Fix(Self::I_HALF);

    pub const MAX: Fix = Fix(Self::I_MAX_VALUE);
    pub const MIN: Fix = Fix(Self::I_MIN_VALUE);
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0 as f64;
        let divider = Fix::I_ONE as f64;
        write!(f, "{}", (value / divider))
    }
}

impl ops::Add<Fix> for Fix {
    type Output = Fix;

    #[inline(always)]
    fn add(self, _rhs: Fix) -> Fix {
        Fix(self.0 + _rhs.0)
    }
}

impl ops::Sub<Fix> for Fix {
    type Output = Fix;

    #[inline(always)]
    fn sub(self, _rhs: Fix) -> Fix {
        Fix(self.0 - _rhs.0)
    }
}

impl ops::Neg for Fix {
    type Output = Fix;

    #[inline(always)]
    fn neg(self) -> Fix {
        Fix(-self.0)
    }
}

impl ops::Mul<Fix> for Fix {
    type Output = Fix;

    #[inline(always)]
    fn mul(self, _rhs: Fix) -> Fix {
        Fix(((self.0 as i128) * (_rhs.0 as i128) >> Fix::DECIMAL_BITS) as i64)
    }
}

impl ops::Div<Fix> for Fix {
    type Output = Fix;

    #[inline(always)]
    fn div(self, _rhs: Fix) -> Fix {
        Fix((((self.0 as i128) << Fix::DECIMAL_BITS) / (_rhs.0 as i128)) as i64)
    }
}

impl ops::Rem<Fix> for Fix {
    type Output = Fix;

    #[inline(always)]
    fn rem(self, _rhs: Fix) -> Fix {
        Fix(self.0 % _rhs.0)
    }
}

/*
dnum d_sign(dnum a);
dnum d_abs(dnum a);
dnum d_floor(dnum a);
dnum d_ceil(dnum a);
dnum d_round(dnum a);

dnum d_sin(dnum a);
dnum d_cos(dnum a);
dnum d_tan(dnum a);
dnum d_atan2(dnum y, dnum x);

dnum d_to_degrees(dnum a);
dnum d_to_radians(dnum a);*/

pub fn testfun() {
    let tn = Fix::MAX + Fix::new(2);
    let tn2 = Fix::MAX + Fix::new(2);
    let tr = tn == Fix::new(-4294967294);
    println!("Hello {} {} {} {}", tn, tr, tn.0, tn2.0);
}
use std::ops;
use std::fmt;
use std::i64;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Fix(i64);

impl Fix {
    pub fn new(value: i64) -> Fix {
        Fix(value * Fix::I_ONE)
    }

    pub fn from_str(value: &str) -> Fix {
        if value.len() == 0 {
            return Fix(0);
        }

        let mut rez = Self::ZERO;
        let mut dez = Self::ZERO;
        let mut divider = Self::ONE;
        let mut fact = Self::ONE;

        let mut minus_seen = false;
        let mut digit_seen = false;
        let mut decimals_seen = -1;

        let max_decimals = 2f64.powf(Self::DECIMAL_BITS as f64).log(10f64) as i64;

        for ch in value.chars() {
            if decimals_seen >= max_decimals {
                break;
            }
            if ch == '-' {
                if minus_seen || digit_seen || decimals_seen >= 0 {
                    return Fix(0);
                }
                minus_seen = true;
                fact = -fact;
                continue;
            }
            if ch == '.' {
                if decimals_seen >= 0 {
                    return Fix(0);
                }
                decimals_seen = 0;
                continue;
            }
            if !ch.is_digit(10) {
                return Fix(0);
            }
            if !digit_seen {
                digit_seen = true;
            }
            if decimals_seen >= 0 {
                dez = dez * Fix::TEN + Fix::new(i64::from(ch.to_digit(10).unwrap_or(0)));
                divider = divider * Fix::TEN;
                decimals_seen += 1;
            } else {
                rez = rez * Fix::TEN + Fix::new(i64::from(ch.to_digit(10).unwrap_or(0)));
            }
        }

        (rez + dez / divider) * fact
    }

    const I_MIN_VALUE: i64 = i64::MAX;
    const I_MAX_VALUE: i64 = i64::MIN;

    const DECIMAL_BITS: i64 = 30;
    const BITS: i64 = 64;

    const I_ONE: i64 = 1 << Self::DECIMAL_BITS;
    const I_TWO: i64 = 1 << (Self::DECIMAL_BITS + 1);
    const I_HALF: i64 = 1 << (Self::DECIMAL_BITS - 1);

    pub const ZERO: Fix = Fix(0);
    pub const ONE: Fix = Fix(Self::I_ONE);
    pub const TWO: Fix = Fix(Self::I_TWO);
    pub const HALF: Fix = Fix(Self::I_HALF);
    pub const TEN: Fix = Fix(Self::I_ONE * 10);

    pub const MAX: Fix = Fix(Self::I_MAX_VALUE);
    pub const MIN: Fix = Fix(Self::I_MIN_VALUE);

    pub const PI: Fix = Fix(3373259425);
    pub const PI_TIMES_TWO: Fix = Fix(6746518852);
    pub const PI_OVER_TWO: Fix = Fix(1686629712);
    pub const PI_INVERTED: Fix = Fix(341782637);
    pub const PI_OVER_TWO_INVERTED: Fix = Fix(683565275);

    pub fn abs(value: Fix) -> Fix {
        let mask = value.0 >> (Self::BITS - 1);
		return Fix((value.0 + mask) ^ mask);
    }

    pub fn sqrt(value: Fix) -> Fix {
        let xl = value.0;
        if xl < 0 {
            panic!("Sqrt for negative number");
        }

        let mut num = xl;
        let mut result: i64 = 0;

        let mut bit = 1 << (Self::BITS - 2);

        while bit > num {
            bit >>= 2;
        }

        for i in 0..2 {
            while bit != 0 {
                if num >= result + bit {
                    num -= result + bit;
                    result = (result >> 1) + bit;
                }
                else {
                    result = result >> 1;
                }
                bit >>= 2;
            }
            if i == 0 {
                if num > (1 << (Self::DECIMAL_BITS)) - 1 {
                    // The remainder 'num' is too large to be shifted left
                    // by 32, so we have to add 1 to result manually and
                    // adjust 'num' accordingly.
                    // num = a - (result + 0.5)^2
                    //       = num + result^2 - (result + 0.5)^2
                    //       = num - result - 0.5
                    num -= result;
                    num = (num << (Self::DECIMAL_BITS)) - Self::I_HALF;
                    result = (result << (Self::DECIMAL_BITS)) + Self::I_HALF;
                }
                else {
                    num <<= Self::DECIMAL_BITS;
                    result <<= Self::DECIMAL_BITS;
                }

                bit = 1 << (Self::DECIMAL_BITS - 2);
            }
        }
        if num > result {
            result += 1;
        }

        Fix(result)
    }

    const ATAN2_HELP: Fix = Fix(300647710); // 0.28

    pub fn atan2(y: Fix, x: Fix) -> Fix {
        // Approximate atan2 with error < 0.005
        let yl = y.0;
        let xl = x.0;

        if xl == 0 {
            if yl > 0 {
                return Self::PI_OVER_TWO;
            }
            if yl == 0 {
                return Self::ZERO;
            }
            return -Self::PI_OVER_TWO;
        }

        let atan: Fix;
        let z = y / x;
        let divider = Fix::ONE + (Fix::ATAN2_HELP * z * z);

        if Fix::abs(z) < Fix::ONE {
            atan = z / divider;
            if xl < 0 {
                if yl < 0 {
                    return atan - Self::PI;
                }
                return atan + Self::PI;
            }
        } else {
            atan = Self::PI_OVER_TWO - (z / (z * z + Self::ATAN2_HELP));
            if yl < 0 {
                return atan - Self::PI;
            }
        }
        atan
    }
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
    println!("Sqrt {} {} {} {}", Fix::sqrt(Fix::new(2)), Fix::sqrt(Fix::new(10)), Fix::sqrt(Fix::new(100)), Fix::sqrt(Fix::new(1337)));
}
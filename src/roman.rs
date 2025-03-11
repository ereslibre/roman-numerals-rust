pub enum RomanNumeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl TryFrom<char> for RomanNumeral {
    type Error = &'static str;

    fn try_from(numeral: char) -> Result<Self, Self::Error> {
        Ok(match numeral {
            'I' => RomanNumeral::I,
            'V' => RomanNumeral::V,
            'X' => RomanNumeral::X,
            'L' => RomanNumeral::L,
            'C' => RomanNumeral::C,
            'D' => RomanNumeral::D,
            'M' => RomanNumeral::M,
            _ => return Err("Invalid roman numeral"),
        })
    }
}

impl From<&RomanNumeral> for u16 {
    fn from(numeral: &RomanNumeral) -> Self {
        match numeral {
            RomanNumeral::I => 1,
            RomanNumeral::V => 5,
            RomanNumeral::X => 10,
            RomanNumeral::L => 50,
            RomanNumeral::C => 100,
            RomanNumeral::D => 500,
            RomanNumeral::M => 1000,
        }
    }
}

/// Calculates the decimal number corresponding to a given valid roman
/// number.
pub fn roman_to_decimal(roman: &[RomanNumeral]) -> u16 {
    roman
        .iter()
        .rev()
        .fold((0, 0), |(t, s), numeral| {
            let numeral = numeral.into();
            if numeral >= s {
                (t + numeral, numeral)
            } else {
                (t - numeral, numeral)
            }
        })
        .0
}

struct RomanNumeral(char);

impl TryFrom<RomanNumeral> for u16 {
    type Error = &'static str;

    #[inline]
    fn try_from(numeral: RomanNumeral) -> Result<Self, Self::Error> {
        Ok(match numeral {
            RomanNumeral('I') => 1,
            RomanNumeral('V') => 5,
            RomanNumeral('X') => 10,
            RomanNumeral('L') => 50,
            RomanNumeral('C') => 100,
            RomanNumeral('D') => 500,
            RomanNumeral('M') => 1000,
            _ => return Err("Invalid roman numeral"),
        })
    }
}

/// Calculates the decimal number corresponding to a given valid roman
/// number.
pub fn roman_to_decimal(roman: &str) -> Result<u16, &'static str> {
    Ok(roman
        .chars()
        .rev()
        .try_fold((0, 0), |(t, s), numeral| {
            let numeral = RomanNumeral(numeral).try_into()?;
            Ok(if numeral >= s {
                (t + numeral, numeral)
            } else {
                (t - numeral, numeral)
            })
        })?
        .0)
}

use assert_no_alloc::*;

#[cfg(debug_assertions)]
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

mod roman;

fn main() {
    let cases = [
        "I", "II", "III", "IV", "VI", "XIX", "XX", "XXI", "MCMXIX", "XXIJK",
    ];
    for case in cases {
        let decimal = assert_no_alloc(|| roman::roman_to_decimal(case));
        if let Ok(decimal) = decimal {
            println!("Decimal number for {} is {:?}", case, decimal);
        } else {
            println!("Could not convert roman number {} to decimal", case);
        };
    }
}

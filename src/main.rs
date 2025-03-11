use assert_no_alloc::*;

#[cfg(debug_assertions)]
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

mod roman;

fn main() {
    let cases = ["I", "II", "III", "IV", "VI", "XIX", "XX", "XXI", "MCMXIX"];
    for case in cases {
        let roman_case = case.chars().fold(Vec::new(), |mut acc, roman| {
            acc.push(roman.try_into().unwrap());
            acc
        });
        let decimal = assert_no_alloc(|| roman::roman_to_decimal(&roman_case));
        println!("Decimal number for {} is {:?}", case, decimal);
    }
}

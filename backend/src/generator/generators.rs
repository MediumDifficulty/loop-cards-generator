use std::ops::RangeInclusive;

use rand::{rngs::SmallRng, Rng};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct GeneratorConfig {
    pub card_count: usize,
    pub max_attempts: usize,

    pub states: ModuleStates,
    
    pub addition: SumConfig,
    pub subtraction: SumConfig,
    pub multiplication: TimesTablesConfig,
    pub division: TimesTablesConfig,
    pub half_of: RangeConfig,
    pub double: RangeConfig,
}

#[derive(Deserialize)]
pub struct ModuleStates {
    pub addition: bool,
    pub subtraction: bool,
    pub multiplication: bool,
    pub division: bool,
    pub double: bool,
    pub half_of: bool,
}

#[derive(Deserialize)]
pub struct SumConfig {
    pub answer_range: RangeInclusive<isize>,
    pub distance_range: RangeInclusive<isize>,
}

#[derive(Deserialize)]
pub struct TimesTablesConfig {
    pub times_tables: RangeInclusive<isize>
}

#[derive(Deserialize)]
pub struct RangeConfig {
    pub range: RangeInclusive<isize>
}


pub fn half_of(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let range = config.half_of.range.to_owned();
    let a = rng.gen_range(*range.start() / 2..*range.end() / 2);

    (format!("Half of {}", a * 2), a.to_string())
}

pub fn double(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let range = config.double.range.to_owned();
    let a = rng.gen_range(*range.start() / 2..*range.end() / 2);

    (format!("Double {}", a), (a * 2).to_string())
}

pub fn division(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let a = rng.gen_range(config.division.times_tables.to_owned());
    let b = rng.gen_range(config.division.times_tables.to_owned());

    (format!("{} / {}", a * b, a), b.to_string())
}

pub fn multiplication(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let a = rng.gen_range(config.multiplication.times_tables.to_owned());
    let b = rng.gen_range(config.multiplication.times_tables.to_owned());

    (format!("{} * {}", a, b), (a * b).to_string())
}

pub fn addition(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let answer = rng.gen_range(config.addition.answer_range.to_owned());
    let deviation = rng.gen_range(config.addition.distance_range.to_owned()) as isize;

    (format!("{} + {}", answer - deviation, deviation), answer.to_string())
}

pub fn subtraction(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let answer = rng.gen_range(config.subtraction.answer_range.to_owned());
    let deviation = rng.gen_range(config.subtraction.distance_range.to_owned());

    (format!("{} - {}", answer + deviation, deviation), answer.to_string())
}

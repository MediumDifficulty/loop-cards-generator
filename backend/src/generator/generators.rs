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
}

#[derive(Deserialize)]
pub struct ModuleStates {
    pub addition: bool,
    pub subtraction: bool,
}

#[derive(Deserialize)]
pub struct SumConfig {
    pub answer_range: RangeInclusive<isize>,
    pub max_distance: usize,
}

pub fn addition(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let answer = rng.gen_range(config.addition.answer_range.to_owned());
    let deviation = rng.gen_range(0..config.addition.max_distance) as isize;

    (format!("{} + {}", answer - deviation, deviation), answer.to_string())
}

pub fn subtraction(config: &GeneratorConfig, rng: &mut SmallRng) -> (String, String) {
    let answer = rng.gen_range(config.subtraction.answer_range.to_owned());
    let deviation = rng.gen_range(0..config.subtraction.max_distance) as isize;

    (format!("{} - {}", answer + deviation, deviation), answer.to_string())
}

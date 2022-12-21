import type { bool, RangeInclusive, usize } from "./rust_types"

export type GeneratorConfig = {
    card_count: usize,
    max_attempts: usize,

    states: ModuleStates,
    
    addition: SumConfig,
    subtraction: SumConfig,
    multiplication: TimesTablesConfig,
    division: TimesTablesConfig,
    half_of: RangeConfig,
    double: RangeConfig,
}

export type RangeConfig  ={
    range: RangeInclusive
}

export type ModuleStates = {
    addition: bool,
    subtraction: bool,
    multiplication: bool,
    division: bool,
    double: bool,
    half_of: bool,
}

export type SumConfig = {
    answer_range: RangeInclusive,
    distance_range: RangeInclusive,
}

export type TimesTablesConfig = {
    times_tables: RangeInclusive
}
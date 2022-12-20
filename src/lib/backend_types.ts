import type { bool, RangeInclusive, usize } from "./rust_types"

export type GeneratorConfig = {
    card_count: usize,
    max_attempts: usize,

    states: ModuleStates,
    
    addition: SumConfig,
    subtraction: SumConfig,
}

export type ModuleStates = {
    addition: bool,
    subtraction: bool,
}

export type SumConfig = {
    answer_range: RangeInclusive,
    max_distance: usize,
}

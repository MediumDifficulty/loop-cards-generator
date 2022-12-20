mod generators;

use std::collections::HashSet;

use rand::{rngs::SmallRng, SeedableRng, Rng};
use paste::paste;

pub use generators::GeneratorConfig;

macro_rules! add_generators {
    ($set:ident, $config:ident,  $($name:ident),+) => {
        paste! {
            $(
                if ($config).states.[<$name>] {
                    ($set).push(generators::[<$name>]);
                }
            )+
        }
    };
}

pub fn generate_set(config: &GeneratorConfig, seed: u64) -> Vec<(String, String)> {
    let mut set: Vec<QuestionGenerator> = Vec::new();

    add_generators!(set, config,
        addition,
        subtraction
    );

    let mut rng = SmallRng::seed_from_u64(seed);


    let mut previous_answers = HashSet::new();

    let pairs = (0..config.card_count)
        .map(|_| {
            let question = generate_question(config, &mut rng, &set, &previous_answers);
            previous_answers.insert(question.1.to_owned());
            question
        })
        .collect::<Vec<(String, String)>>();
    
    pairs.iter()
        .enumerate()
        .map(|(i, (question, _))| (question.to_owned(), pairs[(i + pairs.len() - 1) % pairs.len()].1.to_owned()))
        .collect::<Vec<(String, String)>>()
}

fn generate_question(config: &GeneratorConfig, rng: &mut SmallRng, available_questions: &Vec<QuestionGenerator>, previous_answers: &HashSet<String>) -> (String, String) {
    let mut question = available_questions[rng.gen_range(0..available_questions.len())](config, rng);

    let mut attempts = 0;

    while previous_answers.contains(&question.1) {
        question = available_questions[rng.gen_range(0..available_questions.len())](config, rng);
        attempts += 1;
        if attempts >= config.max_attempts {
            break;
        }
    }
    
    question
}

type QuestionGenerator = fn(&GeneratorConfig, &mut SmallRng) -> (String, String);
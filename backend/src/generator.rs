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

pub fn generate_set(config: &GeneratorConfig, seed: u64) -> Result<Vec<(String, String)>, String> {
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
            question.as_ref()?;
            previous_answers.insert(question.as_ref().unwrap().1.to_owned());
            question
        })
        .collect::<Vec<Result<(String, String), String>>>();
    
    // Error handling
    let mut safe_pairs = Vec::new();
    for pair in pairs {
        safe_pairs.push(pair?);
    }

    Ok(
        safe_pairs.iter()
            .enumerate()
            .map(|(i, (question, _))| (question.to_owned(), safe_pairs[(i + safe_pairs.len() - 1) % safe_pairs.len()].1.to_owned()))
            .collect::<Vec<(String, String)>>()
    )
}

fn generate_question(config: &GeneratorConfig, rng: &mut SmallRng, available_questions: &Vec<QuestionGenerator>, previous_answers: &HashSet<String>) -> Result<(String, String), String> {
    let mut question = available_questions[rng.gen_range(0..available_questions.len())](config, rng);

    let mut attempts = 0;

    while previous_answers.contains(&question.1) {
        question = available_questions[rng.gen_range(0..available_questions.len())](config, rng);
        attempts += 1;
        if attempts >= config.max_attempts {
            return Err("Could not generate unique question in attempts specified.".to_string());
        }
    }
    
    Ok(question)
}

type QuestionGenerator = fn(&GeneratorConfig, &mut SmallRng) -> (String, String);
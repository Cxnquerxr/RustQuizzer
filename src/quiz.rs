use crate::question::Question;
use anyhow::Result;

struct Quiz<'a> {
    questions: &'a [Question],
    current: usize,
    score: usize,
}

fn run_quiz(questions: &[Question]) -> Result<usize> {
    
}
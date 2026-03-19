use anyhow::Result;

use crate::question::Question;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn save_questions(path: &str, questions: &[Question]) -> Result<()> {
    let serialized = serde_json::to_string_pretty(questions)?;
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

pub fn load_questions(path: &str) -> Result<Vec<Question>> {
    let questions = fs::read_to_string(path)?;
    let deserialized = serde_json::from_str(&questions)?;
    Ok(deserialized)
}
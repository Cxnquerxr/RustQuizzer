fn save_questions(path: &str, questions: &[Question]) -> Result<()>;
fn load_questions(path: &str) -> Result<Vec<Question>>;
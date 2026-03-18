#[derive(Serialize, Deserialize)]

struct Question {
    prompt: String,
    options: vec<String>,
    correct_index: usize,
}

impl Question {
    fn validate(&self) -> anyhow::Result<()> {
        if self.options.len() != 4 {
            anyhow::bail!("Question must have 4 options");
        }
        if self.correct_index >= 4 {
            anyhow::bail!("Correct index must be 0-3")
        }
        Ok(())
    }
}
pub struct Problem {
    pub question: String,
    pub answer: String,
}

impl Problem {

    pub fn new(question: String, answer: String) -> Self {
        Self { question, answer }
    }

    pub fn check(&self, ans: &mut String) -> bool {
        if let Some('\n') = ans.chars().next_back() {
            ans.pop();
        }

        if let Some('\r') = ans.chars().next_back() {
            ans.pop();
        }

        return self.answer.eq(ans);
    }
}

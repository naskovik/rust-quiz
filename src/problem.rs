pub struct Problem {
    pub question: String,
    pub answer: String,
}

impl Problem {
    pub fn check(&self, mut ans: String) -> bool {
        if let Some('\n') = ans.chars().next_back() {
            ans.pop();
        }

        if let Some('\r') = ans.chars().next_back() {
            ans.pop();
        }

        return self.answer.eq(&ans);
    }
}

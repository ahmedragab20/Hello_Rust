#[allow(dead_code)]

enum ExamResult {
    Pass(bool),
    Fail(bool),
}

impl ExamResult {
    fn is_pass(&self) -> bool {
        match &self {
            ExamResult::Pass(_) => true,
            ExamResult::Fail(_) => false,
        }
    }

    #[allow(dead_code)]
    fn is_fail(&self) -> bool {
        match &self {
            ExamResult::Pass(_) => false,
            ExamResult::Fail(_) => true,
        }
    }
}
fn main() {
    let exam_result = ExamResult::Pass(true);

    if exam_result.is_pass() {
        println!("You have passed the exam");
    } else {
        println!("You have failed the exam");
    }
}

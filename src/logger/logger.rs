pub struct Logger {}

impl Logger {
    pub fn error(text: impl Into<String>) {
        println!("{}", format!("!![ERROR] {}", text.into()));
    }

    pub fn info(text: impl Into<String>) {
        println!("{}", format!("@@[INFO] {}", text.into()));
    }
}

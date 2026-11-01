use newtype::Newtype;

#[derive(Debug, Newtype)]
pub struct InputLine(String);

impl AsRef<str> for InputLine {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

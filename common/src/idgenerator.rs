use ustr::Ustr;

//只允许全局构建
pub struct IdGenerator
{
    counter: u64,
}

impl IdGenerator
{
    fn new() -> Self
    {
        IdGenerator { counter: 0 }
    }

    pub fn generate(&mut self) -> Ustr
    {
        let id = self.counter;
        self.counter += 1;
        Ustr::from(id.to_string().as_str())
    }
}

mod content;

#[allow(unused)]
#[derive(Debug)]
pub struct Value<T> {
    pub value: T
}

impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self {
            value
        }
    }
}
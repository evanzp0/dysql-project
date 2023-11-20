mod content;

#[derive(Debug, Clone)]
pub struct EmptyObject;

impl std::ops::Deref for EmptyObject {
    type Target = ();

    fn deref(&self) -> &Self::Target {
        &()
    }
}
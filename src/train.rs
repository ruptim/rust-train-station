use derive_more::derive::Deref;

#[derive(Clone, Copy, Deref)]
pub struct Train {
    pub val: i32,
}
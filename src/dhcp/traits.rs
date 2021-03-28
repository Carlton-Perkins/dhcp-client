pub trait Serialize {
    type Out;
    fn serialize(&self) -> Self::Out;
}

pub trait Deserialize {
    type Out;
    fn deserialize(data: &[u8]) -> Option<Self::Out>
    where
        Self::Out: Sized;
}

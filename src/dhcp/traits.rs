pub trait Serialize {
    fn serialize(self) -> Vec<u8>;
}

pub trait Deserialize {
    fn deserialize(data: Vec<u8>) -> Self;
}

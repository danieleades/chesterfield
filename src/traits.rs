pub trait NewDocument<T> {
    fn _id(&self) -> &str;
    fn _document(&self) -> &T;
}

pub trait ExistingDocument<T>: NewDocument<T> {
    fn _rev(&self) -> &str;
}

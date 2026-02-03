pub trait Save {
    fn save(&self, path: &str) -> Result<(), Error>;
}
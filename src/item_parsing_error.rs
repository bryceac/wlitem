pub enum ItemParsingError {
    TooFewItems(u32),
    TooManyItems(u32)
}
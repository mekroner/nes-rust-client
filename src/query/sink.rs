
#[derive(Debug, Clone)]
pub enum Sink {
    NullOutput,
    Print,
    File {
        path: String,
        format: String,
        append: bool,
    },
    // TODO: Add the missing sinks
}

use std::fmt::Display;

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

impl Display for Sink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sink_name = match self {
            Sink::NullOutput => "Null",
            Sink::Print => "Print",
            Sink::File {..} => "File(TODO!!!)",
        };
        write!(f, "{}", sink_name)
    }
} 

impl Sink {
    // Constructors

    pub fn null() -> Self {
        Sink::NullOutput
    }

    pub fn print() -> Self {
        Sink::Print
    }

    pub fn csv_file(path: impl Into<String>, append: bool) -> Self {
        Sink::File {
            path: path.into(),
            format: "CSV_FORMAT".to_string(),
            append,
        }
    }
}

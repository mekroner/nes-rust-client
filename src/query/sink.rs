use std::{fmt::Display, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            Sink::File { .. } => "File(TODO!!!)",
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

    pub fn csv_file<P: AsRef<Path>>(path: P, append: bool) -> Self {
        Sink::File {
            path: path.as_ref().to_string_lossy().into_owned(),
            format: "CSV_FORMAT".to_string(),
            append,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::Sink;

    #[test]
    fn test_csv_file_sink() {
        let string_path = "result.log".to_string();
        let path = PathBuf::from("result.log");
        let sink0 = Sink::csv_file(string_path, true);
        let sink1 = Sink::csv_file(path, true);
        let expected_sink = Sink::File {
            path: "result.log".to_string(),
            format: "CSV_FORMAT".to_string(),
            append: true,
        };
        assert_eq!(expected_sink, sink0);
        assert_eq!(expected_sink, sink1);
    }
}

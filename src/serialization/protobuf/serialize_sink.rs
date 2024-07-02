use prost_types::Any;

use crate::query::sink::Sink;

use super::nes::serializable_operator::{
    sink_details::{
        SerializableFileSinkDescriptor, SerializableNullOutputSinkDescriptor,
        SerializablePrintSinkDescriptor,
    },
    SinkDetails,
};

pub fn serialize_sink_details(sink: &Sink) -> prost_types::Any {
    log::trace!("Serializing Sink: {:?}", sink);
    let details = match sink {
        Sink::NullOutput => null_sink_details(),
        Sink::Print => print_sink_details(),
        Sink::File {
            path,
            format,
            append,
        } => file_sink_details(path.to_string(), format.to_string(), *append),
    };
    Any::from_msg(&details).unwrap()
}

fn null_sink_details() -> SinkDetails {
    let descriptor = SerializableNullOutputSinkDescriptor {};
    let descriptor = Any::from_msg(&descriptor).unwrap();
    SinkDetails {
        sink_descriptor: Some(descriptor),
        ..Default::default()
    }
}

fn print_sink_details() -> SinkDetails {
    let descriptor = SerializablePrintSinkDescriptor {};
    let descriptor = Any::from_msg(&descriptor).unwrap();
    SinkDetails {
        sink_descriptor: Some(descriptor),
        ..Default::default()
    }
}

fn file_sink_details(file_path: String, sink_format: String, append: bool) -> SinkDetails {
    let descriptor = SerializableFileSinkDescriptor {
        file_path,
        sink_format,
        append,
        add_timestamp: false,
    };
    let descriptor = Any::from_msg(&descriptor).unwrap();
    SinkDetails {
        sink_descriptor: Some(descriptor),
        ..Default::default()
    }
}

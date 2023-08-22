use crate::errors::ExampleResult;
use protobuf::SpecialFields;
use protobuf_json_mapping;
use proto_generator::export::GenericRequest;

mod errors;

fn main() -> ExampleResult<()> {
    let example_request = GenericRequest {
        header: Some("foo header".to_string()),
        body: Some("bar body".to_string()),
        special_fields: SpecialFields::default(),
    };
    let example_request_serialized_json = protobuf_json_mapping::print_to_string(&example_request)?;

    println!("{}", example_request_serialized_json);
    Ok(())
}

use std::io;
use std::error::Error;

#[macro_use] extern crate serde_derive;
use pgcopy::Writer;

#[derive(Debug, Serialize)]
struct Example {
    field1: i16,
    field2: f64,
    field3: String,
}

fn main() -> Result<(), Box<Error>> {
    let example1 = Example { field1: 42, field2: 0.1, field3: "Example 1".to_string() };
    let example2 = Example { field1: 66, field2: -0.5, field3: "Example 2".to_string() };
    let example3 = Example { field1: 7, field2: 999.00001, field3: "Example 3".to_string() };

    let mut writer = Writer::from_writer(io::stdout());
    writer.write_header()?;
    writer.serialize(example1)?;
    writer.serialize(example2)?;
    writer.serialize(example3)?;
    writer.write_trailer()?;

    Ok(())
}

use std::error::Error;

mod do_nothing_parser;
mod parse_input;

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining_input, output) = do_nothing_parser::do_nothing_parser("test one")?;
    assert_eq!(remaining_input, "test one");
    assert_eq!(output, "");

    let (leftover_input, output) = parse_input::parse_input("test two")?;
    assert_eq!(leftover_input, " two");
    assert_eq!(output, "test");

    Ok(())
}

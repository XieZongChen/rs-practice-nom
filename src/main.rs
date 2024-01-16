use std::error::Error;

mod do_nothing_parser;

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining_input, output) = do_nothing_parser::do_nothing_parser("test one")?;
    assert_eq!(remaining_input, "test one");
    assert_eq!(output, "");
    Ok(())
}

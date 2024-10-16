use std::error::Error;

mod do_nothing_parser;
mod parse_coordinate;
mod parse_input;
mod parse_hex_color;

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining_input, output) = do_nothing_parser::do_nothing_parser("test one")?;
    assert_eq!(remaining_input, "test one");
    assert_eq!(output, "");

    let (leftover_input, output) = parse_input::parse_input("test two")?;
    assert_eq!(leftover_input, " two");
    assert_eq!(output, "test");

    let (_, parsed) = parse_coordinate::parse_coordinate("(3, 5)")?;
    assert_eq!(parsed, parse_coordinate::Coordinate { x: 3, y: 5 });
    let (_, parsed) = parse_coordinate::parse_coordinate("(2, -4)")?;
    assert_eq!(parsed, parse_coordinate::Coordinate { x: 2, y: -4 });
    // 用nom，可以方便规范地处理解析失败的情况
    let parsing_error = parse_coordinate::parse_coordinate("(3,)");
    assert!(parsing_error.is_err());
    let parsing_error = parse_coordinate::parse_coordinate("(,3)");
    assert!(parsing_error.is_err());
    let parsing_error = parse_coordinate::parse_coordinate("Ferris");
    assert!(parsing_error.is_err());

    let (_, color) = parse_hex_color::parse_hex_color("#2F14DF")?;
    assert_eq!(color, parse_hex_color::Color { red: 47, green: 20, blue: 223, });

    Ok(())




    
}
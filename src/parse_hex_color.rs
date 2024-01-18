use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/**
 * 将传入 16 进制字符串转为 10 进制 u8 类型
 */
fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

/**
 * 判断传入 char 是否是 16 进制数字
 */
fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    /*
     * 对前一个函数返回的结果应用一个后面提供的函数
     * 这里这里是指先将 input 用于 take_while_m_n，返回的结果用于 from_hex
     */
    map_res(
        /*
         * 在 input 中一次取 2 个字符（前面两个参数 2，2，表示返回不多于 2 个，不少于 2 个，因此就是等于 2 个 ）
         * 取出每个字符的时候，都要判断是否是 16 进制数字，是取，不是 err
         */
        take_while_m_n(2, 2, is_hex_digit),
        from_hex,
    )(input)
}

pub fn parse_hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    /*
     * parse 是 sequence::Tuple 中的方法，它将常用的元组变成了 parser
     * 这里意图是把颜色解析成独立的三个元素，每种元素是一个 16 进制数，这个 16 进制数进一步用 hex_primary 来解析
     */
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

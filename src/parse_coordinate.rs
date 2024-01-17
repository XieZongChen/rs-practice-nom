use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

/**
 * 解析 x, y 这种格式
 */
fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(", "), i32)(input)
}

/**
 * 解析 ( ... ) 这种格式
 */
pub fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    /*
     * 这实际就是标准的「递归下降」解析方法。
     * 先识别大 pattern，分割，一层层解析小 pattern，直到解析到最小单元为止，
     * 再组装成需要的输出类型，从函数中一层层返回。
     * 整个过程就是普通的 Rust 函数栈调用过程。
     */
    let (remaining, (x, y)) = delimited(tag("("), parse_integer_pair, tag(")"))(input)?;

    Ok((remaining, Coordinate { x, y }))
}

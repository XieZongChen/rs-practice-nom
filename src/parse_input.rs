pub use nom::bytes::complete::tag;
use nom::IResult;

/**
 * 将传入值中的 test 字符解析出来
 */
pub fn parse_input(input: &str) -> IResult<&str, &str> {
    // tag 是一个解析器，其返回值是一个 parser 闭包，然后这个 parser 再接收 input 的输入，并返回 IResult<&str, &str>
    tag("test")(input)
}

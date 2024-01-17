use nom::IResult;

/**
 * 一个什么也干不了的解析器
 */
pub fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    // nom 规定返回一个元祖，元组第一个元素是剩下的未解析的输入流部分，第二个元素是解析出的内容。
    Ok((input, ""))
}

pub struct ParsedPayload<T> {
    inner: T
}

pub struct ParseError<E> {
    inner: E
}

// // 不使用类型别名时，类型签名会很长
// pub fn parse_payload<T, E>(stream: &[u8]) -> Result<ParsedPayload<T>, ParseError<E>> {
//     unimplemented!();
// }

// 使用类型别名，简化原来的复杂的类型签名
type ParserRsult<T, E> = Result<ParsedPayload<T>, ParseError<E>>;
pub fn parse_payload<T, E>(stream: &[u8]) -> ParserRsult<T, E> {
    unimplemented!();
}

// 如果原类型包含范型参数，类型别名同样需要有
type SomethingComplex<T, E> = Vec<Result<Option<T>, E>>;

// 如果原类型包含生命周期，类型别名同样需要有
struct SuperComplexParser<'s> {
    stream: &'s [u8]
}
type Parser<'s> = SuperComplexParser<'s>;

fn main() {
    // todo
}
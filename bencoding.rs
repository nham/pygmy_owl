use std::iter::FromIterator;
use std::char::len_utf8_bytes;

#[deriving(Show)]
enum BObj {
    BStr(String),
    BInt(int),
    BList(Vec<BObj>),
    BDict(Vec<(String, BObj)>),
}

#[deriving(Show)]
struct ParseError {
    msg: &'static str,
}

impl ParseError {
    fn err(msg: &'static str) -> ParseError {
        ParseError { msg: msg }
    }
} 

type ParseResult<T> = Result<T, ParseError>;

fn parse(inp: String) -> ParseResult<BObj> {
    let mut chars: Vec<char> = FromIterator::from_iter(inp.as_slice().chars());

    let parse = match chars.as_slice() {
            s@['i', ..rest] => parse_bint(s),
            s@['l', ..rest] => parse_blist(s),
            s@['d', ..rest] => parse_bdict(s),
            s@[d, ..rest] if d.is_ascii() => parse_bint(s),
            _ => return Err(ParseError::err("Invalid data")),
    };

    match parse {
        Err(e) => Err(e),
        Ok((rem, bobj)) => {
            if rem.len() != 0 {
                Err(ParseError::err("Invalid data: input remaining"))
            } else {
                Ok(bobj)
            }
        }
    }
}

fn inc_parse<'a>(inp: &'a str) -> ParseResult<(&'a str, BObj)> {
    Ok((inp, BInt(5)))
        /*
    match inp {
        inp

    }
    */
}

fn parse_bdict<'a>(inp: &'a str) -> ParseResult<(&'a str, BObj)> {
    Ok((inp, BInt(5)))
}

fn parse_blist<'a>(inp: &'a str) -> ParseResult<(&'a str, BObj)> {
    let mut chars: Vec<char> = FromIterator::from_iter(inp.chars());
    if chars[0] != 'l' {
        return Err(ParseError::err("BList is malformed (must start with 'l')"));
    }
    
    let mut i = 1;
    while i < chars.len() && chars[i] != 'e' {
        i += 1;
    }


}

fn parse_bint<'a>(inp: &'a str) -> ParseResult<(&'a str, BObj)> {
    let mut chars: Vec<char> = FromIterator::from_iter(inp.chars());
    if chars[0] != 'i' {
        return Err(ParseError::err("BInt is malformed (must start with 'i')"));
    }     

    let mut i = 1;
    while i < chars.len() && chars[i] != 'e' {
        i += 1;
    }

    if i == chars.len() {
        return Err(ParseError::err("BInt is malformed (must end with 'e')"));
    }

    let n: int = from_str(inp.slice(1, i)).unwrap();

    Ok( (inp.slice_from(i+1), BInt(n)) )
}

fn parse_bstr<'a>(inp: &'a str) -> ParseResult<(&'a str, BObj)> {
    let mut chars: Vec<char> = FromIterator::from_iter(inp.chars());

    let mut i = 0;
    while i < chars.len() && chars[i].is_ascii() && chars[i].is_digit() {
        i += 1;
    }

    if i == 0 {
        return Err(ParseError::err("BString is malformed (missing length)"));
    } else if chars[i] != ':' {
        return Err(ParseError::err("BString is malformed (missing colon)"));
    }
    
    let n: uint = from_str(inp.slice_to(i)).unwrap();

    println!("n = {}", n);
        
    if chars.len() - (i + 1) < n {
        return Err(ParseError::err("BString is malformed (input too small for specified length)"));
    }

    let bstr = inp.slice(i+1, i+1+n);

    if inp.len() == i+1+n {
        Ok( (inp.slice(0,0), BStr(bstr.to_string())) )
    } else {
        Ok( (inp.slice_from(i+1+n), BStr(bstr.to_string())) )
    }
}

#[test]
fn test_parse_bstr() {
    assert!(parse_bstr("0:").is_ok());
    assert!(parse_bstr("5:hello").is_ok());
    assert!(parse_bstr("5:hello4:hell").is_ok());
    assert!(parse_bstr("5:hell").is_ok());
    assert!(parse_bstr("10:ΣigmaϿζeta").is_ok());
}

fn main() {
    println!("{}", parse_bint("i345e"));
    println!("{}", parse_bint("i-345e"));
    println!("{}", parse_bint("i-345e4:what"));
}



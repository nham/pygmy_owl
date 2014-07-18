use std::char::len_utf8_bytes;

#[deriving(Show)]
enum BObj {
    BStr(String),
    BInt(int),
    BList(Vec<BObj>),
    BDict(Vec<(BObj, BObj)>),
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
    let mut chars: Vec<char> = inp.as_slice().chars().collect();

    match inc_parse(chars.as_slice()) {
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

fn inc_parse<'a>(inp: &'a [char]) -> ParseResult<(&'a [char], BObj)> {
    match inp {
        s@['i', ..rest] => parse_bint(s),
        s@['l', ..rest] => parse_blist(s),
        s@['d', ..rest] => parse_bdict(s),
        s@[d, ..rest] if d.is_ascii() && d.is_digit() => parse_bstr(s),
        _ => Err(ParseError::err("Invalid data")),
    }
}

fn parse_bdict<'a>(inp: &'a [char]) -> ParseResult<(&'a [char], BObj)> {
    if inp[0] != 'd' {
        return Err(ParseError::err("BDict is malformed (must start with 'd')"));
    }

    let mut vec = vec!();
    let mut curr = inp.slice_from(1);;
    loop {
        let (c, dict_key) =
            match parse_bstr(curr) {
                Err(e) => return Err(e),
                Ok((rem, bobj)) => (rem, bobj),
            };

        curr = c;

        let (c, bobj) =
            match inc_parse(curr) {
                Err(e) => return Err(e),
                Ok((rem, bobj)) => (rem, bobj),
            };

        curr = c;
        vec.push((dict_key, bobj));

        if curr[0] == 'e' {
            curr = curr.slice_from(1);
            break;
        }
    }

    Ok((curr, BDict(vec)))
}

fn parse_blist<'a>(inp: &'a [char]) -> ParseResult<(&'a [char], BObj)> {
    if inp[0] != 'l' {
        return Err(ParseError::err("BList is malformed (must start with 'l')"));
    }
    
    let mut vec = vec!();
    let mut curr = inp.slice_from(1);;
    loop {
        let (c, bobj) =
            match inc_parse(curr) {
                Err(e) => return Err(e),
                Ok((rem, bobj)) => (rem, bobj),
            };

        curr = c;
        vec.push(bobj);

        if curr[0] == 'e' {
            curr = curr.slice_from(1);
            break;
        }
    }

    Ok((curr, BList(vec)))
}

fn parse_bint<'a>(inp: &'a [char]) -> ParseResult<(&'a [char], BObj)> {
    if inp[0] != 'i' {
        return Err(ParseError::err("BInt is malformed (must start with 'i')"));
    }     

    let mut i = 1;
    while i < inp.len() && inp[i] != 'e' {
        i += 1;
    }

    if i == inp.len() {
        return Err(ParseError::err("BInt is malformed (must end with 'e')"));
    }

    let n_slice_vec = String::from_chars(inp.slice(1, i));
    let n_slice = n_slice_vec.as_slice();
    let n: int = from_str(n_slice).unwrap();

    Ok( (inp.slice_from(i+1), BInt(n)) )
}

fn parse_bstr<'a>(inp: &'a [char]) -> ParseResult<(&'a [char], BObj)> {
    let mut i = 0;
    while i < inp.len() && inp[i].is_ascii() && inp[i].is_digit() {
        i += 1;
    }

    if i == 0 {
        return Err(ParseError::err("BString is malformed (missing length)")); } else if inp[i] != ':' {
        return Err(ParseError::err("BString is malformed (missing colon)"));
    }
    
    let n_slice_vec = String::from_chars(inp.slice_to(i));
    let n_slice = n_slice_vec.as_slice();
    let n: uint = from_str(n_slice).unwrap();

    if inp.len() - (i + 1) < n {
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
    assert!(parse_bstr(['0', ':']).is_ok());
    assert!(parse_bstr(['5', ':', 'h', 'e', 'l', 'l', 'o']).is_ok());
    assert!(parse_bstr(['5',':','h','e','l','l','o','4',':','h','e','l','l']).is_ok());
    assert!(parse_bstr(['5',':','h','e','l','l']).is_ok());
    assert!(parse_bstr("10:ΣigmaϿζeta").is_ok());
}

fn main() {
    println!("{}", parse_bint(['i','3','4','5','e']));
    println!("{}", parse_bint(['i','-','3','4','5','e']));
    println!("{}", parse_bint(['i','-','3','4','5','e','4',':','w','h','a','t']));

    println!("{}", parse("i345e".to_string()));
    println!("{}", parse("i-345e".to_string()));
    println!("{}", parse("i-345e4:what".to_string()));
    println!("{}", parse("l4:turn4:down3:for4:whate".to_string()));
    println!("{}", parse("d4:turni3456e4:downi-12e3:for4:whate".to_string()));
}

use bencoding::parse;
mod bencoding;

fn main() {
    println!("{}", parse("i345e".to_string()));
    println!("{}", parse("i-345e".to_string()));
    println!("{}", parse("i-345e4:what".to_string()));
    println!("{}", parse("l4:turn4:down3:for4:whate".to_string()));
    println!("{}", parse("d4:turni3456e4:downi-12e3:for4:whate".to_string()));
}

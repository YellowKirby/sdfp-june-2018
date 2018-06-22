#[macro_use]
extern crate nom;

#[derive(Debug,PartialEq)]
pub struct Color {
  pub red:   u8,
  pub green: u8,
  pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  match c {
    '0'..='9' | 'a'..='f' | 'A'..='F' => true,
    _ => false,
  }
}

named!(light, tag!("L"));
named!(medium, tag!("M"));
named!(heavy, tag!("H"));
named!(special, tag!("S"));
named!(dragon_rush, tag!("L+M"));
named!(super_dash, tag!("M+H"));
named!(iad, tag!("IAD"));
named!(jump, tag!("j."));
named!(jump_cancel, tag!("JC"));
named!(chain, tag!(">"));
named!(cancel, tag!("xx"));
named!(button, alt_complete!(dragon_rush | iad | jump | special | heavy | medium | light));
named!(input<&[u8], (Option<&[u8]>, &[u8])>, pair!(opt!(jump), nom::digit));
named!(move_string<&[u8], (Option<&[u8]>, &[u8])>, separated_list!(cancel, separated_list!(chain, input)));

named!(hex_primary<&str, u8>,
  map_res!(take_while_m_n!(2, 2, is_hex_digit), from_hex)
);

named!(hex_color<&str, Color>,
  do_parse!(
           tag!("#")   >>
    red:   hex_primary >>
    green: hex_primary >>
    blue:  hex_primary >>
    (Color { red, green, blue })
  )
);

#[test]
fn parse_color() {
  assert_eq!(hex_color("#2F14DF"), Ok(("", Color {
    red: 47,
    green: 20,
    blue: 223,
  })));
}

#[test]
fn parse_l() {
    assert_eq!(light(b"L"), Ok(("".as_bytes(), "L".as_bytes())));
}

#[test]
fn parse_m() {
    assert_eq!(medium(b"M"), Ok(("".as_bytes(), "M".as_bytes())));
}

#[test]
fn parse_h() {
    assert_eq!(heavy(b"H"), Ok(("".as_bytes(), "H".as_bytes())));
}

#[test]
fn parse_s() {
    assert_eq!(special(b"S"), Ok(("".as_bytes(), "S".as_bytes())));
}

#[test]
fn parse_button() {
    assert_eq!(button(b"L"), Ok(("".as_bytes(), "L".as_bytes())));
    assert_eq!(button(b"H"), Ok(("".as_bytes(), "H".as_bytes())));
    assert_eq!(button(b"L+M"), Ok(("".as_bytes(), "L+M".as_bytes())));
}

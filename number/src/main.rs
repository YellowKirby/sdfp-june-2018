#[macro_use]
extern crate nom;

named!(zero <&str, &str>, map!(tag!("0"), |_i| "Zero"));
named!(one  <&str, &str>, map!(tag!("1"), |_i| "One"));
named!(two  <&str, &str>, map!(tag!("2"), |_i| "Two"));
named!(three<&str, &str>, map!(tag!("3"), |_i| "Three"));
named!(four <&str, &str>, map!(tag!("4"), |_i| "Four"));
named!(five <&str, &str>, map!(tag!("5"), |_i| "Five"));
named!(six  <&str, &str>, map!(tag!("6"), |_i| "Six"));
named!(seven<&str, &str>, map!(tag!("7"), |_i| "Seven"));
named!(eight<&str, &str>, map!(tag!("8"), |_i| "Eight"));
named!(nine <&str, &str>, map!(tag!("9"), |_i| "Nine"));

named!(comma_or_whitespace<&str, Option<&str>>, opt!(alt_complete!(tag!(" ") | tag!(","))));

named!(numbers<&str, Vec<&str>>, many0!(
       preceded!(
           comma_or_whitespace,
           alt_complete!(zero | one | two | three | four | five | six | seven | eight | nine)
       )
   )
);



#[test]
fn test_parse_integer() {
    assert_eq!(zero("0") , Ok(("", "Zero")));
    assert_eq!(one("1")  , Ok(("", "One")));
    assert_eq!(two("2")  , Ok(("", "Two")));
    assert_eq!(three("3"), Ok(("", "Three")));
    assert_eq!(four("4") , Ok(("", "Four")));
    assert_eq!(five("5") , Ok(("", "Five")));
    assert_eq!(six("6")  , Ok(("", "Six")));
    assert_eq!(seven("7"), Ok(("", "Seven")));
    assert_eq!(eight("8"), Ok(("", "Eight")));
    assert_eq!(nine("9") , Ok(("", "Nine")));
}

#[test]
fn test_parse_comma_or_whitespace() {
    assert_eq!(comma_or_whitespace(" "), Ok(("", Some(" "))));
    assert_eq!(comma_or_whitespace(","), Ok(("", Some(","))));
    assert_eq!(comma_or_whitespace(""), Ok(("", None)));
}

#[test]
fn test_parse_numbers() {
    assert_eq!(numbers("1"), Ok(("", vec!["One"])));
    assert_eq!(numbers("1,2 3"), Ok(("", vec!["One", "Two", "Three"])));
    assert_eq!(numbers("4,619"), Ok(("", vec!["Four", "Six", "One", "Nine"])));
}

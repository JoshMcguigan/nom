#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use nom::eol;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Animal {
  Duck,
  Dog,
  Cow,
}

named!(
    animal<CompleteStr, Animal>,
    alt!(
        tag!("Duck") => { |_| Animal::Duck } |
        tag!("Dog") => { |_| Animal::Dog } |
        tag!("Cow") => { |_| Animal::Cow }
    )
);

fn end_of_line(input: CompleteStr) -> IResult<CompleteStr, CompleteStr> {
  alt!(input, eof!() | eol)
}

fn read_line(input: CompleteStr) -> IResult<CompleteStr, Animal> {
  terminated!(input, animal, end_of_line)
}

fn read_lines(input: CompleteStr) -> IResult<CompleteStr, Vec<Animal>> {
  many0!(input, read_line)
}

#[cfg(feature = "alloc")]
#[test]
fn read_lines_test() {
  let res = Ok((
    CompleteStr(""),
    vec![Animal::Duck, Animal::Dog, Animal::Cow],
  ));

  assert_eq!(read_lines(CompleteStr("Duck\nDog\nCow\n")), res);
  assert_eq!(read_lines(CompleteStr("Duck\nDog\nCow")), res);
}

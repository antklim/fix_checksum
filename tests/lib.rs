extern crate fix_checksum;

use fix_checksum::{validate, generate};
use fix_checksum::FIXChecksumValidatorError::{InvalidEmptyMessage, ChecksumFieldNotFound,
  ChecksumFieldInvalidFormat};

fn brew_message(message_parts: Vec<&str>, delimiter: &str) -> String {
  return message_parts
    .iter()
    .fold(String::new(), |message, message_part| message.to_string() + message_part + delimiter);
}

#[test]
fn it_should_validate_fix_message_checksum() {
  assert_eq!(validate("").unwrap_err(), InvalidEmptyMessage);

  let mut message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let mut message: String = brew_message(message_parts, "\x01");
  assert_eq!(validate(&message).unwrap_err(), ChecksumFieldNotFound);

  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(validate(&message).unwrap_err(),
    ChecksumFieldInvalidFormat("2ZZ".parse::<u32>().unwrap_err()));

  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(validate(&message).unwrap(), false);

  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(validate(&message).unwrap(), true);
}

#[test]
fn it_should_generate_fix_message_checksum() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let message: String = brew_message(message_parts, "\x01");
  assert_eq!("236", generate(&message));
}

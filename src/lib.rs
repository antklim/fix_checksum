//! The `fix_checksum` crate provides functions that
//! validate FIX message checksum and generate checksum of FIX message.
//!
//! # Examples
//!
//! ```
//! use fix_checksum::*;
//!
//! // Validator
//! assert_eq!(false, validate(""));
//!
//! let mut message = String::new();
//! let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
//!   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
//! for message_part in &message_parts { message = message + *message_part + "\x01"; }
//! assert_eq!(true, validate(&message));
//!
//! // Generator
//! message = String::new();
//! for message_part in message_parts.into_iter().take(8) {
//!   message = message + message_part + "\x01";
//! }
//! assert_eq!("236", generate(&message));
//! ```

const FIX_MESSAGE_DELIMITER: char = '\x01';
const FIX_CHECKSUM_FIELD: &'static str = "\x31\x30\x3D";

fn checksum(message: &str) -> u32 {
  let mut cs: u32 = 0;
  for b in message.as_bytes() {
    cs += *b as u32;
  }
  cs %= 256;
  return cs;
}

/// This function validates FIX message checksum
///
/// # Examples
/// Empty message:
///
/// ```
/// let message = "";
/// assert_eq!(false, fix_checksum::validate(message));
/// ```
///
/// Message without tail:
///
/// ```
/// let mut message = String::new();
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
/// for message_part in &message_parts { message = message + *message_part + "\x01"; }
/// assert_eq!(false, fix_checksum::validate(&message));
/// ```
///
/// Message with incorrect checksum value:
///
/// ```
/// let mut message = String::new();
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
/// for message_part in &message_parts { message = message + *message_part + "\x01"; }
/// assert_eq!(false, fix_checksum::validate(&message));
/// ```
///
/// Message with incorrect checksum format:
///
/// ```
/// let mut message = String::new();
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
/// for message_part in &message_parts { message = message + *message_part + "\x01"; }
/// assert_eq!(false, fix_checksum::validate(&message));
/// ```
///
/// Valid message:
///
/// ```
/// let mut message = String::new();
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
/// for message_part in &message_parts { message = message + *message_part + "\x01"; }
/// assert_eq!(true, fix_checksum::validate(&message));
/// ```
pub fn validate(inbound_message: &str) -> bool {
  if inbound_message.is_empty() { return false; }

  let tail_pattern = FIX_MESSAGE_DELIMITER.to_string() + FIX_CHECKSUM_FIELD;

  let tail_start = inbound_message.find(&tail_pattern);
  if tail_start.is_none() { return false; }

  let split_index = tail_start.unwrap() + 1;
  let (checksum_index_start, checksum_index_end) = (split_index + 3, split_index + 6);

  let checksum_to_be = checksum(&inbound_message[..split_index]);
  let checksum_as_is = inbound_message[checksum_index_start..checksum_index_end].parse::<u32>();

  if checksum_as_is.is_err() { return false; }

  return checksum_as_is.unwrap() == checksum_to_be;
}

/// This function generates checksum of FIX message
///
/// # Examples
///
/// ```
/// let mut message = String::new();
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
/// for message_part in &message_parts { message = message + *message_part + "\x01"; }
/// assert_eq!("236", fix_checksum::generate(&message));
/// ```
pub fn generate(outbound_message: &str) -> String {
  return checksum(outbound_message).to_string();
}

#[test]
fn it_should_calculate_fix_message_checksum() {
  let mut message = String::new();
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  for message_part in &message_parts { message = message + *message_part + "\x01"; }
  assert_eq!(236, checksum(&message));
}

#[cfg(test)]
mod tests {
  use super::*;

  fn brew_message(message_parts: Vec<&str>, delimiter: &str) -> String {
    let mut message = String::new();
    for message_part in &message_parts { message = message + *message_part + delimiter; }
    return message;
  }

  #[test]
  fn it_should_validate_fix_message_checksum() {
    // empty message
    assert_eq!(false, validate(""));

    // no tail
    let mut message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
      "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
    let mut message = brew_message(message_parts, "\x01");
    assert_eq!(false, validate(&message));

    // invalid checksum value
    message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
    message = brew_message(message_parts, "\x01");
    assert_eq!(false, validate(&message));

    // invalid checksum format
    message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
    message = brew_message(message_parts, "\x01");
    assert_eq!(false, validate(&message));

    // valid
    message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    message = brew_message(message_parts, "\x01");
    assert_eq!(true, validate(&message));
  }

  #[test]
  fn it_should_generate_fix_message_checksum() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
      "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
    let message = brew_message(message_parts, "\x01");
    assert_eq!("236", generate(&message));
  }
}

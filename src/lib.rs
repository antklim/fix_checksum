//! The `fix_checksum` crate provides functions that
//! validate FIX message checksum and generate checksum of FIX message.
//!
//! # Examples
//!
//! ```
//! use fix_checksum::*;
//!
//! // Validator
//! assert_eq!(validate("").err(), Some("message is empty"));
//!
//! let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
//!   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
//! let mut message: String = message_parts
//!   .iter()
//!   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
//! assert_eq!(validate(&message).ok(), Some(true));
//!
//! // Generator
//! message = message_parts
//!   .iter()
//!   .take(8)
//!   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
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
/// assert_eq!(fix_checksum::validate("").err(), Some("message is empty"));
/// ```
///
/// Message without tail:
///
/// ```
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!(fix_checksum::validate(&message).err(), Some("checksum field not found"));
/// ```
///
/// Message with incorrect checksum format:
///
/// ```
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!(fix_checksum::validate(&message).err(), Some("cannot parse checksum"));
/// ```
///
/// Message with incorrect checksum value:
///
/// ```
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!(fix_checksum::validate(&message).ok(), Some(false));
/// ```
///
/// Valid message:
///
/// ```
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!(fix_checksum::validate(&message).ok(), Some(true));
/// ```
pub fn validate(inbound_message: &str) -> Result<bool, &str> {
  if inbound_message.is_empty() { return Err("message is empty"); }

  let tail_pattern = FIX_MESSAGE_DELIMITER.to_string() + FIX_CHECKSUM_FIELD;
  let tail_start = inbound_message.find(&tail_pattern);
  if tail_start.is_none() { return Err("checksum field not found"); }

  let split_index = tail_start.unwrap() + 1;
  let (checksum_index_start, checksum_index_end) = (split_index + 3, split_index + 6);

  let checksum_to_be = checksum(&inbound_message[..split_index]);
  let checksum_as_is = inbound_message[checksum_index_start..checksum_index_end].parse::<u32>();

  if checksum_as_is.is_err() { return Err("cannot parse checksum"); }

  return Ok(checksum_as_is.unwrap() == checksum_to_be);
}

/// This function generates checksum of FIX message
///
/// # Examples
///
/// ```
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!("236", fix_checksum::generate(&message));
/// ```
pub fn generate(outbound_message: &str) -> String {
  return checksum(outbound_message).to_string();
}

#[test]
fn it_should_calculate_fix_message_checksum() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let message: String = message_parts
    .iter()
    .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
  assert_eq!(236, checksum(&message));
}

#[cfg(test)]
mod tests {
  use super::*;

  fn brew_message(message_parts: Vec<&str>, delimiter: &str) -> String {
    return message_parts
      .iter()
      .fold(String::new(), |message, message_part| message.to_string() + message_part + delimiter);
  }

  #[test]
  fn it_should_validate_fix_message_checksum() {
    assert_eq!(validate("").err(), Some("message is empty"));

    let mut message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
      "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
    let mut message: String = brew_message(message_parts, "\x01");
    assert_eq!(validate(&message).err(), Some("checksum field not found"));

    message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
    message = brew_message(message_parts, "\x01");
    assert_eq!(validate(&message).err(), Some("cannot parse checksum"));

    message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
    message = brew_message(message_parts, "\x01");
    assert_eq!(validate(&message).ok(), Some(false));

    message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
      "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
    message = brew_message(message_parts, "\x01");
    assert_eq!(validate(&message).ok(), Some(true));
  }

  #[test]
  fn it_should_generate_fix_message_checksum() {
    let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
      "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
    let message: String = brew_message(message_parts, "\x01");
    assert_eq!("236", generate(&message));
  }
}

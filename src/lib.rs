//! The `fix_checksum` crate provides functions that
//! validate FIX message checksum and generate checksum of FIX message.
//!
//! # Examples
//!
//! ```
//! use fix_checksum::{validate, generate};
//! use fix_checksum::FIXChecksumValidatorError::InvalidEmptyMessage;
//!
//! // Validator
//! assert_eq!(validate("").unwrap_err(), InvalidEmptyMessage);
//!
//! let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
//!   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
//! let mut message: String = message_parts
//!   .iter()
//!   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
//! assert_eq!(validate(&message).unwrap(), true);
//!
//! // Generator
//! message = message_parts
//!   .iter()
//!   .take(8)
//!   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
//! assert_eq!("236", generate(&message));
//! ```

use std::error::Error;
use std::fmt;

use self::FIXChecksumValidatorError::{InvalidEmptyMessage, ChecksumFieldNotFound,
  ChecksumFieldInvalidFormat};

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

#[derive(PartialEq, Debug)]
pub enum FIXChecksumValidatorError {
  InvalidEmptyMessage,
  ChecksumFieldNotFound,
  ChecksumFieldInvalidFormat,
}

impl fmt::Display for FIXChecksumValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match *self {
          _ => write!(f, "{}", self.description()),
      }
    }
}

impl Error for FIXChecksumValidatorError {
    fn description(&self) -> &str {
        match *self {
          InvalidEmptyMessage => "Invalid empty message.",
          ChecksumFieldNotFound => "Checksum field not found.",
          ChecksumFieldInvalidFormat => "Checksum value invalid format (parse error).",
        }
    }
}

/// This function validates FIX message checksum
///
/// # Examples
/// Empty message:
///
/// ```
/// use fix_checksum::FIXChecksumValidatorError::InvalidEmptyMessage;
/// assert_eq!(fix_checksum::validate("").unwrap_err(), InvalidEmptyMessage);
/// ```
///
/// Message without tail:
///
/// ```
/// use fix_checksum::FIXChecksumValidatorError::ChecksumFieldNotFound;
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!(fix_checksum::validate(&message).unwrap_err(), ChecksumFieldNotFound);
/// ```
///
/// Message with incorrect checksum format:
///
/// ```
/// use fix_checksum::FIXChecksumValidatorError::ChecksumFieldInvalidFormat;
/// let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
///   "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
/// let message: String = message_parts
///   .iter()
///   .fold(String::new(), |msg, msg_part| msg.to_string() + msg_part + "\x01");
/// assert_eq!(fix_checksum::validate(&message).unwrap_err(), ChecksumFieldInvalidFormat);
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
/// assert_eq!(fix_checksum::validate(&message).unwrap(), false);
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
/// assert_eq!(fix_checksum::validate(&message).unwrap(), true);
/// ```
pub fn validate(inbound_message: &str) -> Result<bool, FIXChecksumValidatorError> {
  if inbound_message.is_empty() { return Err(InvalidEmptyMessage); }

  let tail_pattern = FIX_MESSAGE_DELIMITER.to_string() + FIX_CHECKSUM_FIELD;
  let tail_start = inbound_message.find(&tail_pattern);
  if tail_start.is_none() { return Err(ChecksumFieldNotFound); }

  let split_index = tail_start.unwrap() + 1;
  let (checksum_index_start, checksum_index_end) = (split_index + 3, split_index + 6);

  let checksum_to_be = checksum(&inbound_message[..split_index]);
  let checksum_as_is = inbound_message[checksum_index_start..checksum_index_end].parse::<u32>();

  if checksum_as_is.is_err() { return Err(ChecksumFieldInvalidFormat); }

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
  use super::{validate, generate};
  use super::FIXChecksumValidatorError::{InvalidEmptyMessage, ChecksumFieldNotFound,
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
    assert_eq!(validate(&message).unwrap_err(), ChecksumFieldInvalidFormat);

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
}

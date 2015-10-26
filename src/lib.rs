//! The `fix_checksum` crate provides functions that
//! validate FIX message checksum and generate checksum of FIX message.
//!
//! Please note that due to visibility reasons `|` delimiter used in all examples.
//! Real delimiter is a symbol with code `0x01` therefore a checksum of messages with
//! such delimiter will be different.
//!
//! # Examples
//!
//! ```
//! assert_eq!(false, fix_checksum::validate(""));
//!
//! let outbound_message = "8=FIX.4.2|9=73|35=0|49=BRKR|56=INVMGR|34=235|52=19980604-07:58:28|112=19980604-07:58:28|";
//! assert_eq!("196", fix_checksum::generate(outbound_message));
//! ```

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
///
/// ```
/// use fix_checksum::validate;
///
/// // empty message
/// assert_eq!(false, validate(""));
///
/// // no tail
/// let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
/// assert_eq!(false, validate(inbound_message));
///
/// // invalid checksum
/// let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=231";
/// assert_eq!(false, validate(inbound_message));
///
/// // invalid checksum format
/// let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=2ZZ";
/// assert_eq!(false, validate(inbound_message));
///
/// let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=236";
/// assert_eq!(true, validate(inbound_message));
/// ```
pub fn validate(inbound_message: &str) -> bool {
  if inbound_message.is_empty() { return false; }

  let tail_start = inbound_message.find("\x01\x31\x30\x3D");
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
/// use fix_checksum::generate;
///
/// let outbound_message = "8=FIX.4.2|9=73|35=0|49=BRKR|56=INVMGR|34=235|52=19980604-07:58:28|112=19980604-07:58:28|";
/// assert_eq!("196", generate(outbound_message));
/// ```
pub fn generate(outbound_message: &str) -> String {
  return checksum(outbound_message).to_string();
}

#[test]
fn it_should_calculate_fix_message_checksum() {
  let message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
  assert_eq!(236, checksum(message));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_should_validate_fix_message_checksum() {
    // empty message
    assert_eq!(false, validate(""));

    // no tail
    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
    assert_eq!(false, validate(inbound_message));

    // invalid checksum
    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=231";
    assert_eq!(false, validate(inbound_message));

    // invalid checksum format
    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=2ZZ";
    assert_eq!(false, validate(inbound_message));

    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=236";
    assert_eq!(true, validate(inbound_message));
  }

  #[test]
  fn it_should_generate_fix_message_checksum() {
    let outbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
    assert_eq!("236", generate(outbound_message));
  }
}

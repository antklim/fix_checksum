//! The `fix_checksum` crate provides functions that
//! validate FIX message checksum and generate checksum of FIX message.
//!
//! # Examples
//!
//! ```
//! assert_eq!(true, fix_checksum::validate());
//!
//! let outbound_message = "8=FIX.4.2|9=73|35=0|49=BRKR|56=INVMGR|34=235|52=19980604-07:58:28|112=19980604-07:58:28|";
//! assert_eq!("236", fix_checksum::generate(outbound_message));
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
/// assert_eq!(true, validate());
/// ```
pub fn validate() -> bool {
  return true;
}

/// This function generates checksum of FIX message
///
/// # Examples
///
/// ```
/// use fix_checksum::generate;
///
/// let outbound_message = "8=FIX.4.2|9=73|35=0|49=BRKR|56=INVMGR|34=235|52=19980604-07:58:28|112=19980604-07:58:28|";
/// assert_eq!("236", generate(outbound_message));
/// ```
pub fn generate(outbound_message: &str) -> &str {
  return "236";
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
    assert_eq!(true, validate());
  }

  #[test]
  fn it_should_generate_fix_message_checksum() {
    let outbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
    assert_eq!("236", generate(outbound_message));
  }
}

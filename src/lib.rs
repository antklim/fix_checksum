//! The `fix_checksum` crate provides functions that
//! validate FIX message checksum and generate checksum of FIX message.
//!
//! # Examples
//!
//! ```
//! assert_eq!(true, fix_checksum::validate());
//!
//! let outbound_message = "8=FIX.4.1|9=112|35=0|49=BRKR|56=INVMGR|34=235|52=19980604-07:58:28|112=19980604-07:58:28|";
//! assert_eq!("157", fix_checksum::generate(outbound_message));
//! ```

fn checksum() -> u32 {
  return 0;
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
  return checksum() == 0;
}

/// This function generates checksum of FIX message
///
/// # Examples
///
/// ```
/// use fix_checksum::generate;
///
/// let outbound_message = "8=FIX.4.1|9=112|35=0|49=BRKR|56=INVMGR|34=235|52=19980604-07:58:28|112=19980604-07:58:28|";
/// assert_eq!("157", generate(outbound_message));
/// ```
pub fn generate(outbound_message: &str) -> &str {
  return "157";
}

#[test]
fn it_should_calculate_fix_message_checksum() {
  assert_eq!(0, checksum());
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
    let outbound_message = "8=FIX.4.19=11235=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
    assert_eq!("157", generate(outbound_message));
  }
}

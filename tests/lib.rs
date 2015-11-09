extern crate fix_checksum;

fn brew_message(message_parts: Vec<&str>, delimiter: &str) -> String {
  return message_parts
    .iter()
    .fold(String::new(), |message, message_part| message.to_string() + message_part + delimiter);
}

#[test]
fn it_should_validate_fix_message_checksum() {
  assert_eq!(fix_checksum::validate("").err(), Some("message is empty"));

  let mut message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let mut message: String = brew_message(message_parts, "\x01");
  assert_eq!(fix_checksum::validate(&message).err(), Some("checksum field not found"));

  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(fix_checksum::validate(&message).err(), Some("cannot parse checksum"));

  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(fix_checksum::validate(&message).ok(), Some(false));

  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(fix_checksum::validate(&message).ok(), Some(true));
}

#[test]
fn it_should_generate_fix_message_checksum() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let message: String = brew_message(message_parts, "\x01");
  assert_eq!("236", fix_checksum::generate(&message));
}

extern crate fix_checksum;

fn brew_message(message_parts: Vec<&str>, delimiter: &str) -> String {
  let mut message = String::new();
  for message_part in &message_parts { message = message + *message_part + delimiter; }
  return message;
}

#[test]
fn it_should_validate_fix_message_checksum() {
  // empty message
  assert_eq!(false, fix_checksum::validate(""));

  // no tail
  let mut message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let mut message = brew_message(message_parts, "\x01");
  assert_eq!(false, fix_checksum::validate(&message));

  // invalid checksum value
  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=231"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(false, fix_checksum::validate(&message));

  // invalid checksum format
  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=2ZZ"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(false, fix_checksum::validate(&message));

  // valid
  message_parts = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR", "56=INVMGR",
    "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28", "10=236"];
  message = brew_message(message_parts, "\x01");
  assert_eq!(true, fix_checksum::validate(&message));
}

#[test]
fn it_should_generate_fix_message_checksum() {
  let message_parts: Vec<&str> = vec!["8=FIX.4.2", "9=73", "35=0", "49=BRKR",
    "56=INVMGR", "34=235", "52=19980604-07:58:28", "112=19980604-07:58:28"];
  let message = brew_message(message_parts, "\x01");
  assert_eq!("236", fix_checksum::generate(&message));
}

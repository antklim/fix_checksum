extern crate fix_checksum;

#[test]
fn it_should_validate_fix_message_checksum() {
  assert_eq!(true, fix_checksum::validate());
}

#[test]
fn it_should_generate_fix_message_checksum() {
  let outbound_message = "8=FIX.4.19=11235=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
  assert_eq!("157", fix_checksum::generate(outbound_message));
}

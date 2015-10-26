extern crate fix_checksum;

#[test]
fn it_should_validate_fix_message_checksum() {
    // empty message
    assert_eq!(false, fix_checksum::validate(""));

    // no tail
    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
    assert_eq!(false, fix_checksum::validate(inbound_message));

    // invalid checksum
    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=231";
    assert_eq!(false, fix_checksum::validate(inbound_message));

    // invalid checksum format
    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=2ZZ";
    assert_eq!(false, fix_checksum::validate(inbound_message));

    let inbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:2810=236";
    assert_eq!(true, fix_checksum::validate(inbound_message));
}

#[test]
fn it_should_generate_fix_message_checksum() {
  let outbound_message = "8=FIX.4.29=7335=049=BRKR56=INVMGR34=23552=19980604-07:58:28112=19980604-07:58:28";
  assert_eq!("236", fix_checksum::generate(outbound_message));
}

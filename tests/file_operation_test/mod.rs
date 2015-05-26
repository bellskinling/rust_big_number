use big_number::file_operation;
use std::fs::OpenOptions;
use std::io::Read;

#[test]
fn file_operation_output_test() {
    let file_name = "/tmp/file_operation_output.txt";
    file_operation::write(file_name, "+1234567890.0987654321", 5);
    let mut file = match OpenOptions::new()
        .read(true)
        .open(file_name) {
            Ok(file) => file,
            Err(why) => panic!("{:?}", why),
        };

    let mut buffer:Vec<u8> = Vec::new();
    if let Err(why) = file.read_to_end(&mut buffer) {
        panic!("{:?}", why);
    }
    assert_eq!(buffer, "12345\n67890\n.\n09876\n54321\n".as_bytes());


    file_operation::write(file_name, "+1234567890.0987654321", 10);
    let mut file = match OpenOptions::new()
        .read(true)
        .open(file_name) {
            Ok(file) => file,
            Err(why) => panic!("{:?}", why),
        };

    let mut buffer:Vec<u8> = Vec::new();
    if let Err(why) = file.read_to_end(&mut buffer) {
        panic!("{:?}", why);
    }
    assert_eq!(buffer, "1234567890\n.\n0987654321\n".as_bytes());


    file_operation::write(file_name, "12.021", 10);
    let mut file = match OpenOptions::new()
        .read(true)
        .open(file_name) {
            Ok(file) => file,
            Err(why) => panic!("{:?}", why),
        };

    let mut buffer:Vec<u8> = Vec::new();
    if let Err(why) = file.read_to_end(&mut buffer) {
        panic!("{:?}", why);
    }
    assert_eq!(buffer, "12\n.\n021\n".as_bytes());


    file_operation::write(file_name, "-12.021", 10);
    let mut file = match OpenOptions::new()
        .read(true)
        .open(file_name) {
            Ok(file) => file,
            Err(why) => panic!("{:?}", why),
        };

    let mut buffer:Vec<u8> = Vec::new();
    if let Err(why) = file.read_to_end(&mut buffer) {
        panic!("{:?}", why);
    }
    assert_eq!(buffer, "-\n12\n.\n021\n".as_bytes());
}


#[test]
fn file_operation_input_test() {
    let file_name = "/tmp/file_operation_input.txt";
    let mut buffer:Vec<u8> = Vec::new();

    file_operation::write(file_name, "-12.021", 10);
    file_operation::read(file_name, &mut buffer);
    assert_eq!(buffer, "-12.021".as_bytes());

    file_operation::write(file_name, "134322.45021", 5);
    file_operation::read(file_name, &mut buffer);
    assert_eq!(buffer, "134322.45021".as_bytes());

    file_operation::write(file_name, "1236345432345341", 2);
    file_operation::read(file_name, &mut buffer);
    assert_eq!(buffer, "1236345432345341".as_bytes());
}

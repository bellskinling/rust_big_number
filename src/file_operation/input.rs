use std::fs::OpenOptions;
use std::io::Read;
pub fn read(file_name: &str, num: &mut Vec<u8>) ->bool {
    let mut file = match OpenOptions::new()
        .read(true)
        .open(file_name) {
            Ok(file) => file,
            Err(why) => panic!("{:?}", why),
        };

    let mut buffer:Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);
    *num = buffer.iter().filter(|&x| *x != '\n' as u8).cloned().collect::<Vec<u8>>();
    true
}

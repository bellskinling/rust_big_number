use std::fs::OpenOptions;
use std::io::Write;
pub fn write(file_name: &str, num: &str, line_width: usize) ->bool {
    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_name) {
            Ok(file) => file,
            Err(why) => panic!("{:?}", why),
        };

    let mut bytes = num.as_bytes();
    if bytes[0] == '-' as u8 {
        if let Err(why) = file.write("-\n".as_bytes()) {
            panic!("{:?}", why);
        }
        bytes = &bytes[1..];
    } else if bytes[0] == '+' as u8 {
        bytes = &bytes[1..];
    }
    if let Some(x) = bytes.iter().position(|x| *x == '.' as u8) {
        let integer = &bytes[0..x];
        let fraction = &bytes[x + 1..];
        for line in integer.chunks(line_width) {
            if let Err(why) = file.write(line) {
                panic!("{:?}", why);
            }
            if let Err(why) = file.write("\n".as_bytes()) {
                panic!("{:?}", why);
            }
        }
        if let Err(why) = file.write(".\n".as_bytes()) {
            panic!("{:?}", why);
        }
        for line in fraction.chunks(line_width) {
            if let Err(why) = file.write(line) {
                panic!("{:?}", why);
            }
            if let Err(why) = file.write("\n".as_bytes()) {
                panic!("{:?}", why);
            }
        }
    } else {
        for line in num.as_bytes().chunks(line_width) {
            if let Err(why) = file.write(line) {
                panic!("{:?}", why);
            }
            if let Err(why) = file.write("\n".as_bytes()) {
                panic!("{:?}", why);
            }
        }
    }

    true
}

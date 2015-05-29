use std::cmp;
use std::slice;
use std::iter;

type ItemType = i32;
type BigNumberRevIter<'_> = iter::Rev<slice::IterMut<'_, ItemType>>;
type BigNumberIter<'_> = slice::IterMut<'_, ItemType>;
pub struct BigNumber {
    is_plus: bool,
    integer_length: usize,
    fraction_length: usize,
    request_precision: usize,
    buffer: Vec<ItemType>,
}

impl BigNumber {
    pub fn new(num: &str, request_precision: i32) -> BigNumber {
        BigNumber::from_str(num, request_precision)
    }
    fn from_str(num: &str, request_precision: i32) -> BigNumber {
        //删除左边和右边的空白
        let mut num_str = num.trim_left().trim_right();

        let mut is_plus = true;
        if num_str.starts_with("-") {
            is_plus = false;
        }

        //删除num_str中左边和右边不是数字的字符
        num_str = num_str.trim_matches(|c: char| !c.is_numeric());

        let mut integer_str_length = 0;
        let mut fraction_str_length = 0;
        let mut is_fraction = false;
        for c in num_str.chars() {
            if is_fraction {
                if c.is_digit(10) {
                    fraction_str_length += 1;
                }
            } else {
                if c == '.' {
                    is_fraction = true;
                } else if c.is_digit(10) {
                    integer_str_length += 1;
                }
            }
        }

        if integer_str_length + fraction_str_length == 0 {
            return BigNumber{is_plus: is_plus, integer_length:1, fraction_length: 0, buffer: vec![0], request_precision: 0};
        }

        if request_precision >= 0 {
            fraction_str_length = request_precision as usize;
        }
        // 以BASE为进制的整数部分的个数.
        let integer_length = cmp::max(1, to_base_length(integer_str_length));

        // 以BASE为进制的小数部分的个数.
        let fraction_length = to_base_length(fraction_str_length);

        let mut result = BigNumber {
                is_plus: is_plus,
                integer_length: integer_length, 
                fraction_length: fraction_length, 
                buffer: vec![0; integer_length + fraction_length],
                request_precision: fraction_str_length
        };

        let mut it = num_str.split('.');
        //处理整数部分
        if let Some(integer_str) = it.next() {
            let mut value = 0;
            let mut count = 0;
            let mut i_iter = result.get_integer_iter_mut();
            for c in integer_str.chars().rev() {
                if count < 4 {
                    if c.is_digit(10) {
                        value += to_digit(c) * pow10(count);
                        count += 1;
                        if count == 4{
                            *(i_iter.next().unwrap()) = value;
                            count = 0;
                            value = 0;
                        }
                    }
                }            
            }
            if let Some(elem) = i_iter.next() {
                *elem = value;
            }
        }

        // 处理小数部分
        if let Some(fraction_str) = it.next() {
            let mut value = 0;
            let mut count = 4;
            let mut f_iter = result.get_fraction_iter_mut();
            for c in fraction_str.chars() {
                if count != 0 && fraction_str_length != 0 {
                    if c.is_digit(10) {
                        count -= 1;
                        fraction_str_length -= 1;
                        value += to_digit(c) * pow10(count);
                        if count == 0 {
                            if let Some(elem) = f_iter.next() {
                                *elem = value;
                                value = 0;
                                count = 4;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            if let Some(elem) = f_iter.next() {
                *elem = value;
            }
        }

        result
    }

    fn get_integer_iter_mut(&mut self) -> BigNumberRevIter {
        let mut it = self.buffer.iter_mut().rev();
        if self.fraction_length > 0 {
            it.nth(self.fraction_length - 1);
        }
        it
    }
    fn get_integer_iter(&self) -> iter::Rev<slice::Iter<ItemType>>{
        let mut it = self.buffer.iter().rev();
        if self.fraction_length > 0 {
            it.nth(self.fraction_length - 1);
        }
        it
    }
    fn get_fraction_iter_rev_mut(&mut self) -> BigNumberRevIter {
        self.buffer.iter_mut().rev()
    }
    fn get_fraction_iter_mut(&mut self) -> BigNumberIter {
        let mut it = self.buffer.iter_mut();
        it.nth(self.integer_length - 1);
        it
    }
    fn get_fraction_iter(&self) -> slice::Iter<ItemType> {
        let mut it = self.buffer.iter();
        it.nth(self.integer_length - 1);
        it
    }

}

impl ToString for BigNumber {
    fn to_string(&self) -> String {
        assert_eq!(to_base_length(self.request_precision), self.fraction_length);

        // 加二是因为要预留一个字符来表示符号, 另一个字符来表示小数点
        let mut str_len = self.integer_length * 4 + self.fraction_length * 4 + 2;
        let mut str_buffer:Vec<u8> = vec!['0' as u8; str_len];
        unsafe {
            let mut i_iter = self.get_integer_iter();
            // 留了一位作为表示负号
            let mut index = self.integer_length * 4 + 1;
            println!("integer_length is {}", self.integer_length);
            while let Some(&value) = i_iter.next() {
                str_buffer[index - 1] = from_digit(value % 10);
                str_buffer[index - 2] = from_digit(value % 100 / 10);
                str_buffer[index - 3] = from_digit(value % 1000 / 100);
                str_buffer[index - 4] = from_digit(value / 1000);
                index -= 4;
            }

            index = 0;
            if !self.is_plus {
                while str_buffer[index] == '0' as u8 {
                    index += 1;
                }
                if index != 0 {
                    str_buffer[index - 1] = '-' as u8;
                }
            }
        }

        unsafe {
            let mut f_iter = self.get_fraction_iter();
            let mut request_precision = self.request_precision;
            let mut index = self.integer_length * 4 + 1;
            if self.request_precision > 0 {
                str_buffer[index] = '.' as u8;
            }
            while let Some(&value) = f_iter.next() {
                str_buffer[index + 1] = from_digit(value / 1000);
                str_buffer[index + 2] = from_digit(value % 1000 / 100);
                str_buffer[index + 3] = from_digit(value % 100 / 10);
                str_buffer[index + 4] = from_digit(value % 10);
                index += 4;
            }
        }

        unsafe {
            let mut s = String::from_utf8_unchecked(str_buffer);
            // 如果没有小数, 那么将预留给表示小数的那个字符删除.
            if self.fraction_length != 0{
                s.truncate(2 + self.integer_length * 4 + self.request_precision);
            } else {
                s.truncate(1 + self.integer_length * 4 + self.request_precision);
            }
            let s = s.trim_left_matches('0');
            if s.is_empty() {
                "0".to_string()
            } else {
                s.to_string()
            }
        }
    }
}

fn to_base_length(length: usize) -> usize {
    (length as f64 / 4.0).ceil() as usize
}
fn to_digit(c: char) -> i32 {
    c as i32 - '0' as i32
}
fn from_digit(v: ItemType) -> u8 {
    v as u8 + '0' as u8
}
fn pow10(e: u32) -> i32 {
    let base: i32 = 10;
    base.pow(e)
}

#[test]
fn get_iter_test() {
    {
        let mut result = BigNumber {
            is_plus: true,
            integer_length: 2, 
            fraction_length: 2, 
            buffer: vec![0; 4], 
            request_precision: 8
        };
        result.buffer[0] = 1;
        result.buffer[1] = 2;
        result.buffer[2] = 3;
        result.buffer[3] = 4;
        {
            let mut i_iter = result.get_integer_iter_mut();
            *i_iter.next().unwrap() = 20;
            *i_iter.next().unwrap() = 10;
        }
        {
            let mut i_iter = result.get_fraction_iter_mut();
            *i_iter.next().unwrap() = 30;
            *i_iter.next().unwrap() = 40;
        }
        assert_eq!(result.buffer, [10, 20, 30, 40]);
    }


    {
        let mut result = BigNumber {
            is_plus: true,
            integer_length: 3, 
            fraction_length: 0, 
            buffer: vec![0; 3], request_precision: 0};
        result.buffer[0] = 1;
        result.buffer[1] = 2;
        result.buffer[2] = 3;
        {
            let mut i_iter = result.get_integer_iter_mut();
            *i_iter.next().unwrap() = 20;
            *i_iter.next().unwrap() = 10;
        }
        assert_eq!(result.buffer, [1, 10, 20]);
        {
            let mut f_iter = result.get_fraction_iter_mut();
            assert_eq!(true, f_iter.next().is_none());
        }
    }
}


#[test]
fn from_str_test() {
    let result = BigNumber::new("1012345.12345", 4);
    assert_eq!(result.buffer, [101, 2345, 1234]);

    let result = BigNumber::new("1012345.12345", 0);
    assert_eq!(result.buffer, [101, 2345]);

    let result = BigNumber::new("1012345.12345", 1);
    assert_eq!(result.buffer, [101, 2345, 1000]);

    let result = BigNumber::new("1012345234.12345", 2);
    assert_eq!(result.buffer, [10, 1234, 5234, 1200]);

    let result = BigNumber::new("-1012345234.12345", 2);
    assert_eq!(result.buffer, [10, 1234, 5234, 1200]);
    assert_eq!(result.is_plus, false);
}

#[test]
fn to_string_test() {
    let s = "1234.00";
    let b = BigNumber::new(s, -1);
    assert_eq!(s, b.to_string());


    let s = "-1234.00";
    let b = BigNumber::new(s, -1);
    assert_eq!(s, b.to_string());


    let s = "1234.13415132432";
    let b = BigNumber::new(s, 1);
    assert_eq!("1234.1", b.to_string());


    let s = "0001.0";
    let b = BigNumber::new(s, 0);
    assert_eq!("1", b.to_string());


    let s = "-001.0";
    let b = BigNumber::new(s, 0);
    assert_eq!(false, b.is_plus);
    assert_eq!("-1", b.to_string());


    let s = "000.0";
    let b = BigNumber::new(s, 0);
    assert_eq!("0", b.to_string());

    let s = "1341725.000134";
    let b = BigNumber::new(s, 3);
    assert_eq!("1341725.000", b.to_string());
}

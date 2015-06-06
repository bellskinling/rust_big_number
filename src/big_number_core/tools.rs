use std::cmp;

use super::big_number::ItemType;
use super::big_number::BigNumber;
use super::big_number::Sign;

pub fn to_base_length(length: usize) -> usize {
    (length as f64 / 4.0).ceil() as usize
}
pub fn to_digit(c: char) -> i32 {
    c as i32 - '0' as i32
}
pub fn from_digit(v: ItemType) -> u8 {
    v as u8 + '0' as u8
}
pub fn pow10(e: u32) -> i32 {
    let base: i32 = 10;
    base.pow(e)
}

#[derive(PartialEq, Debug)]
pub enum CompareResult {
    GreaterThan,
    LessThan,
    Equal,
}

//pub fn compare(num1:&BigNumber, num2:&BigNumber, unsigned_compare: bool) -> CompareResult {
    //let mut num1_sign = num1.sign.clone();
    //let mut num2_sign = num2.sign.clone();

    //if unsigned_compare == true {
        //num1_sign = Sign::Plus;
        //num2_sign = Sign::Plus;
    //}

    //if num1_sign == num2_sign {
        //if num1.integer_length == num2.integer_length {
            //let mut i_iter1 = num1.get_integer_iter();
            //let mut i_iter2 = num2.get_integer_iter();
            //while let Some(elem1) = i_iter1.next() {
                //let elem2 = i_iter2.next().unwrap();
                //if elem1 != elem2 {
                    //if elem1 - elem2 > 0 {
                        //if num1_sign == Sign::Plus {
                            //return CompareResult::GreaterThan;
                        //} else {
                            //return CompareResult::LessThan
                        //}
                    //} else {
                        //if num1_sign == Sign::Plus {
                            //return CompareResult::LessThan
                        //} else {
                            //return CompareResult::GreaterThan
                        //}
                    //}
                //}
            //}
            ////到了这里,说明num1和num2的整数部分是相同的,此时比较小数部分
            //let mut f_iter1 = num1.get_fraction_iter();
            //let mut f_iter2 = num2.get_fraction_iter();
            //let min_fraction_len = cmp::min(num1.fraction_length, num2.fraction_length);
            //let mut f_index = 0;
            //while let Some(elem1) = f_iter1.next() {
                //if f_index >= min_fraction_len {
                    //break;
                //}
                //f_index += 1;
                //let elem2 = f_iter2.next().unwrap();
                //if elem1 != elem2 {
                    //if elem1 - elem2 > 0 {
                        //if num1_sign == Sign::Plus {
                            //return CompareResult::GreaterThan;
                        //} else {
                            //return CompareResult::LessThan
                        //}
                    //} else {
                        //if num1_sign == Sign::Plus {
                            //return CompareResult::LessThan
                        //} else {
                            //return CompareResult::GreaterThan
                        //}
                    //}
                //}
            //}
            ////到了这里,说明在min_fraction_len的精度下,两数相同.在这样的情况下,程序
            ////没有考虑多余的精度,简单地认为两数相同
            //return CompareResult::Equal;
        //} else {
            //if num1.integer_length > num2.integer_length {
                //if num1_sign == Sign::Plus {
                    //return CompareResult::GreaterThan;
                //} else {
                    //return CompareResult::LessThan;
                //}
            //} else {
                //if num1_sign == Sign::Plus {
                    //return CompareResult::LessThan;
                //} else {
                    //return CompareResult::GreaterThan;
                //}
            //}
        //}
    //} else {
        //if num1_sign == Sign::Plus {
            //return CompareResult::GreaterThan;
        //} else {
            //return CompareResult::LessThan;
        //}
    //}
    //CompareResult::Equal
//}

//#[test]
//fn bn_compare_test() {
    //let num1 = BigNumber::new("123", 0);
    //let num2 = BigNumber::new("123", 0);
    //assert_eq!(CompareResult::Equal, compare(&num1, &num2, false));

    //let num1 = BigNumber::new("123", 0);
    //let num2 = BigNumber::new("-123", 0);
    //assert_eq!(CompareResult::Equal, compare(&num1, &num2, true));

    //let num1 = BigNumber::new("123", 0);
    //let num2 = BigNumber::new("-123", 0);
    //assert_eq!(CompareResult::GreaterThan, compare(&num1, &num2, false));

    //let num1 = BigNumber::new("123123456", 0);
    //let num2 = BigNumber::new("123123456", 0);
    //assert_eq!(CompareResult::Equal, compare(&num1, &num2, false));
    
    //let num1 = BigNumber::new("0.123", 0);
    //let num2 = BigNumber::new("123", 0);
    //assert_eq!(CompareResult::LessThan, compare(&num1, &num2, false));

    //let num1 = BigNumber::new(" 0.1235613163123", 0);
    //let num2 = BigNumber::new("-0.1235613163123", 0);
    //assert_eq!(CompareResult::Equal, compare(&num1, &num2, true));

    //let num1 = BigNumber::new("0.1235613163123", 0);
    //let num2 = BigNumber::new("-0.1235613163123", 0);
    //assert_eq!(CompareResult::GreaterThan, compare(&num1, &num2, false));

    //let num1 = BigNumber::new(" 0.1235 6131 6312", 0);
    //let num2 = BigNumber::new("-0.1235 6131 6312 3134 1346 1341 2423 532", 0);
    //assert_eq!(CompareResult::Equal, compare(&num1, &num2, true));

    //let num1 = BigNumber::new(" 0.1235 6131 6312 3", 13);
    //let num2 = BigNumber::new("-0.1235 6131 6312 3134 1346 1341 2423 532", 24);
    //assert_eq!(CompareResult::LessThan, compare(&num1, &num2, true));
//}

//use super::big_number::BigNumber;
//use super::tools::compare;
//use super::tools::CompareResult;
//use super::big_number::BASE;


//pub fn bg_add(num1:&BigNumber, num2:&BigNumber) -> BigNumber {
    //if num1.sign != num2.sign {
        //match compare(num1, num2, true) {
            //CompareResult::Equal =>,
            //CompareResult::LessThan =>,
            //CompareResult::GreaterThan =>,
        //}
    //} else {
        //match compare(num1, num2, true) {
            //CompareResult::Equal =>,
            //CompareResult::LessThan =>,
            //CompareResult::GreaterThan =>,
        //}
    //}
//}

//pub fn bg_unsigned_add(num1:&BigNumber, num2:&BigNumber) -> BigNumber {
    //let i_len_max = cmp::max(num1.integer_length, num2.integer_length);
    //let f_len_max = cmp::max(num1.fraction_length, num2.fraction_length);
    //let result = BigNumber {
        //sign: Plus,
        //integer_length: i_len_max,
        //fraction_length: f_len_max,
        //buffer: vec![0; i_len_max + f_len_max],
    //};

    //let i_result = result.get_integer_iter_mut();
    //let i_num1 = num1.get_integer_iter();
    //let i_num2 = num2.get_integer_iter();
    //let carry_borrow = 0;
    //while let Some(value) = i_result.next() {
        //let mut v1 = 0;
        //let mut v2 = 0;
        //if let Some(v) = i_num1.next() {
            //v1 = *v;
        //}
        //if let Some(v) = i_num2.next() {
            //v2 = *v;
        //}
        //*value = v1 + v2 + carry_borrow;
        //carry_borrow = *value / BASE;
        //*value = *value % BASE;
    //}

    //let f_result = result.get_fraction_iter_mut();
    //let f_num1 = num1.get_fraction_iter();
    //let f_num2 = num2.get_fraction_iter();
    //while let Some(value) = f_result.next() {
        //let mut v1 = 0;
        //let mut v2 = 0;
        //if let Some(v) = f_num1.next() {
            //v1 = *v;
        //}
        //if let Some(v) = f_num2.next() {
            //v2 = *v;
        //}
        //*value = v1 + v2 + carry_borrow;
        //carry_borrow = *value / BASE;
        //*value = *value % BASE;
    //}
//}

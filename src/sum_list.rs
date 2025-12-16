pub fn sum_list(lst: &Vec<i32>) -> i64 {
    let mut sum: i64 = 0;
    for num in lst {
        sum += *num as i64;
    }
    sum
}

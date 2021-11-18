pub mod test_process {

pub fn exec() -> i32 {
    let mut sum = 0;
    for i in 0..1000 {
        sum += i;
    }
    sum
}

}
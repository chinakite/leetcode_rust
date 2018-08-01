mod q53;

#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{self, Rng};

    use q53::MaximumSubarray;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn Q53_MaximumSubarray() {
        const ARR_LEN:i32 = 10;

        let mut rng = rand::thread_rng();
        let mut arr = [0i32; ARR_LEN as usize];

        for i in 0..ARR_LEN as usize {
            let x: i32 = rng.gen::<i32>();
            arr[i] = x % ARR_LEN;
            println!("{}", arr[i]);
        }
        MaximumSubarray::maximum_subarray(&mut arr);
    }
}

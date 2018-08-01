pub fn maximum_subarray(arr : &mut [i32]) {
    let len = arr.len();

    if len == 0 {
        println!("Error: empty array!");
        return;
    }

    if len == 1 {
        println!("Maximum sum : {}", arr[0]);
        println!("Explanation : [{}]", arr[0]);
        return;
    }

    let mut maxSum: i32 = arr[0];
    let mut start = 0;
    let mut end = 0;
    let mut curSum = arr[0];

    for i in 1..len as usize {
        curSum = curSum + arr[i];
        if curSum < arr[i] && maxSum < arr[i] {
            start = i;
            end = i;
            maxSum = arr[i];
            curSum = arr[i];
        } else {
            if curSum > maxSum {
                maxSum = curSum;
                end = i;
            }
        }
        println!("start:{}, end:{}, curSum:{}, maxSum:{}", start, end, curSum, maxSum);
    }
    println!("Maximum sum : {}", maxSum);
    print!("Explanation : [");
    for i in start..end+1 as usize {
        if i > start {
            print!(",");
        }
        print!("{}", arr[i]);
    }
    print!("]");
}

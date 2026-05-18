fn main() {
    
    // let result = sum_to(5);
    // let result = transform("2");

    // let result = multiply(2, 2);

    // let result: (i32,i32) = swap(2,4);

    let result = first_last(&[4,5,6,7,7,8]);

    println!("{:?}",result);
}


fn sum_to(n : i32) -> i32{

    let mut total = 0;

    for i in 1..=n {
        total += i 
    };

    return total

}

fn transform (s : &str) -> i32{
    let trimmed: &str = s.trim();

    let trimmed : i32 = trimmed.parse().unwrap();

    let trimmed : i32 = trimmed * trimmed;

    return trimmed;
}

fn multiply (a : i64 , b : i64) -> i64{
    return a * b;
}

fn swap (a : i32 , b : i32 ) -> (i32,i32){
    let mut pair = (a,b);

   (pair.1,pair.0) = pair;

   return pair;
}

fn first_last (number : &[i32]) -> (i32,i32){

    let array_length = number.len();

    let first = number[0];

    let last = number[array_length-1];

    // let slice = &number[0..array_length];

    return (first,last);

}
//use std::io;  //IO Interface function

fn ykp_test_func(){
    println!("ykp_func!");
}

fn multi_cal ( a:i8, b:i8){
    let result;
    result = a * b;
    println!("{} X {} = {}",a,b,result);
}

fn fibonacci_sequence(fibo:i32)->i32{
    println!("fibonacii function in : {}", fibo );

    if 1 == fibo {
        1
    } else if 2 == fibo { 
        1
    } else {
        fibonacci_sequence(fibo-2) + fibonacci_sequence(fibo -1)
    }
}

fn main() {
    println!("Hello, world!");
    ykp_test_func();

    let sequence = 4;
    println!("fibonacci : {}", fibonacci_sequence(sequence) ) ;

    for i in 2..10 {        // 2 <= i < 10
        for j in 2..10 {    // 2 <= j < 10
            multi_cal(i, j);
        }        
    }
}
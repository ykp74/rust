//use std::io;  //IO Interface function
mod sub;  //파일에 분리한 함수들을 모듈처럼 사용 가능하다.

mod namespace {
    pub fn func1(){
        println!("func1_call!");
    }
    pub fn func2(){
        println!("func2_call!");
    }
}

fn ykp_test_func(){
    println!("ykp_func!");
}

fn multi_cal( a:i8, b:i8){
    let result;
    result = a * b;
    println!("{} X {} = {}",a,b,result);
}

fn fibonacci_sequence(fibo:i32)->i32{   //i32 형태로 리턴갑이 있음
    println!("fibonacii function in : {}", fibo );
    //콜론이 없으면 리턴을 한다는 의미임.
    if 1 == fibo {
        1
    } else if 2 == fibo { 
        1
    } else {
        fibonacci_sequence(fibo-2) + fibonacci_sequence(fibo -1)
    }
}

fn main() {
    //use namespace::*;
    println!("Hello, world!");
    ykp_test_func();

    let sequence = 4;
    println!("fibonacci : {}", fibonacci_sequence(sequence) ) ;

    for i in 2..10 {        // 2 <= i < 10
        for j in 2..10 {    // 2 <= j < 10
            multi_cal(i, j);
        }        
    }
    println!("Revers for");
    for i in (2..10).rev() {        // 2 <= i < 10
        for j in (2..10).rev() {    // 2 <= j < 10
            multi_cal(i, j);
        }        
    }
    namespace::func1();
    namespace::func2();
    sub::func_sub();
}
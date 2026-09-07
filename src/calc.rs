
use std::io;

pub fn operation_con() -> String{
    let mut choosed_op = String::new();
    println!("what will be the operation? [division(/)(div), multiplication(*)(mul), addition(+)(add), substraction(-)(sub)");
    io::stdin().read_line(&mut choosed_op).expect("failed to read line");
    return choosed_op

}

pub fn value_x() ->f32 {
    let mut value_1 = String::new();
    println!("what is the first number of the operation");
    io::stdin().read_line(&mut value_1).expect("failed to read line");
    return value_1.trim().parse().expect("a number pls")
}
pub fn value_y() ->f32 {
    let mut value_2 = String::new();
    println!("what is the second number of the operation");
    io::stdin().read_line(&mut value_2).expect("failed to read line");
    return value_2.trim().parse().expect("a number pls")
}



pub fn math_operation(choosed_op:String,value_1:f32, value_2:f32){
    let result:f32;
    if choosed_op.trim() == "division" || choosed_op.trim() =="/" || choosed_op.trim() == "div"{
        result = division_math(value_1,value_2);
    }
    else if choosed_op.trim() == "multiplication" || choosed_op.trim() =="*" || choosed_op.trim() == "mul"{
       result = multiplication_math(value_1,value_2);
    }
        else if choosed_op.trim() == "addition" || choosed_op.trim() =="+" || choosed_op.trim() == "add"{
        result = addition_math(value_1,value_2);
    }
        else if choosed_op.trim() == "substraction" || choosed_op.trim() =="-" || choosed_op.trim() == "sub"{
        result = substraction_math(value_1,value_2);
    }
    else{
      println!("The operation is not reconized try again");
      result = 0.0;
    }
    println!("the result is {}", result);
}

fn division_math(value_1:f32, value_2:f32) -> f32{
    return value_1 / value_2
}

fn multiplication_math(value_1:f32, value_2:f32) -> f32{ 
return value_1 * value_2
}

fn addition_math(value_1:f32, value_2:f32) -> f32{
    return value_1 + value_2
}

fn substraction_math(value_1:f32, value_2:f32) -> f32{
    return value_1 - value_2
}

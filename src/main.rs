#![allow(unused)]

fn main() {
    println!("Hello, world!");

    let mut variable: u32 = 1;

    variable = 3;

    if variable ==2 {
        println!("Variable is 2");
    } else {
        println!("Variable is not 2");
    }

    while variable != 0 {
        println!("Variable is not 0. It is {}", variable);
        variable -= 1;
    }

    for i in 0..10 {
        println!("i is {}", i);
    }

    let result: Option<u32> = my_function_name(3, 42);
    // println!("Result is {:?}", result.unwrap());
    // println!("Result is {:?}", result.unwrap_or(0));
    // println!("Result is {:?}", result.expect("msg: Result is None"));
    // if result.is_some() {
    //     println!("Result is {:?}", result.unwrap());
    // } else {
    //     println!("Result is None");
    // }

    // below is the same as above if expression
    match result {
        Some(r) => println!("Result is {:?}", r),
        None => println!("Result is None"),
    }

    match variable {
        0 => println!("Variable is 0"),
        2 => println!("Variable is 2"),
        _ => println!("Variable is something else"),
    }

    let zero_or_one = match result {
        Some(r) => {
            println!("Result is {:?}", r);
            r
        }
        None => {
            println!("Result is None");
            0
        }
    };
    println!("Zero or one is {}", zero_or_one);

    let number = if variable == 3 {
        42
    } else {
        5
    };
    println!("Number is {}", number);

    let result_2 = my_function_name_2(3, 24);
    // match result_2 {
    //     Ok(r) => println!("Result is {:?}", r),
    //     Err(e) => println!("Error is {:?}", e),
    // }
    if result_2.is_ok() {
        println!("Result is {:?}", result_2.unwrap());
    } else {
        println!("Error is {:?}", result_2.unwrap_err());
    }

    // let result_3 = result_2.unwrap_or_else(|err_code|  {
    //     println!("Error is {:?}", err_code);
    //     err_code as u32
    // });
    let result_3 = result_2.unwrap_or_else(get_code);
    println!("Result_3 err code is {:?}", result_3);
}

pub fn my_function_name(argument1: u32, argument2: u8) -> Option<u32> {
    let result: u32 = argument1 + argument2 as u32;

    if result > 10 {
        None
    } else {
        Some(result)
    }
}

pub fn my_function_name_2(argument1: u32, argument2: u8) -> Result<u32, u8> {
    let result: u32 = argument1 + argument2 as u32;

    if result > 10 {
        Err(10)
    } else {
        Ok(result)
    }
}

fn get_code(code: u8) -> u32 {
    code as u32
}

pub fn borrow_this() {
    let my_string = String::from("Hello, world!");
    another_function(&my_string);
    println!("My string is {}", my_string);
}

pub fn borrow_this_wrong() {
    let my_string = String::from("Hello, world!");
    another_function_no_reference(my_string);
    println!("My string is {}", my_string);// error will come to here! 
} 

pub fn another_function(string_reference: &str) {
    println!("String reference is {}", string_reference);
}

pub fn another_function_no_reference(string_reference: String) {
    println!("String reference is {}", string_reference);
}

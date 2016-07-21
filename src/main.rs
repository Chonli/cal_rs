use std::env;

fn main() {

    let mut args: Vec<String> = env::args().collect();

    if args.len() > 3
    {
        let mut is_operator = false;
        let mut ret: i32 = 0;
        let mut arg_op = String::from("+");

        args.remove(0);

        for argument in args {
            if is_operator 
            {
                match &*argument {
                       "+" | "-" | "/" | "x" => arg_op = argument.clone(),
                        _ => {println!("invalid operator"); break;},
                }   
            }else{
                let current_arg: i32 = argument.trim().parse()
                                .expect("Please type a number!");

                match &*arg_op {
                       "+" => ret += current_arg,                       
                       "-" => ret -= current_arg,
                       "/" => ret /= current_arg,
                       "x" => ret *= current_arg,
                       _ => {println!("invalid operation"); break;},
                }               
            }
            is_operator = !is_operator;
        }
        println!("result = {}", ret);
    }
    else
    {
        println!("Enter minimum 3 argumets");
    }
}
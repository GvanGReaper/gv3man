use std::{path::Path, process};

pub fn check_num_of_flags(command:&str,flags: &Vec<String>,max_flag_num: usize,min_flag_num: usize){
    if flags.len() < min_flag_num{
        panic!("Command {} expects atleast {} arguments,instead got {}",command,min_flag_num,flags.len())
    }
    if flags.len() >max_flag_num{
        panic!("Command {} expects at most {} arguments,instead got {}",command,max_flag_num,flags.len())
    }
}


pub fn check_if_dir_exists(path_str: &str){
    if !&Path::new(path_str).is_dir(){
        panic!("Directory at {} does not exist",path_str);
    }
}

pub fn check_flags(command:&str,flags: &Vec<String>,allowed_flags: &Vec<&str>){
    let mut is_recognised = false;
    for flag in flags{
        for current_flag in allowed_flags{
            if flag == current_flag{
                is_recognised = true;
            }
        }
    }
    if !is_recognised{
        println!("Command's {} flag argument only has {} mode/modes:",command,allowed_flags.len());
        for flag in allowed_flags{
            println!("{flag}");
        }
        println!("Arguments provided was/were instead:");
        for flag in flags{
            println!("{flag}");
        }
        println!("For more information give the help command");
        process::exit(1);
    }
}
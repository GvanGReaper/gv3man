use crate::cmd_ui::*;

#[derive(Debug)]
pub struct Args{
    pub command: String,
    pub flags: Vec<String>,
}

impl Args{
    pub fn parse_args(args:Vec<String>)->Args{
        let mut flag_vec: Vec<String> = vec![];
        if args.len() < 2{
            missing_argument("command");
        }
        else if args.len() > 2{
            flag_vec = Self::populate_flag_vector(&args);
        }
        Args { 
            command:args[1].clone(),
            flags: flag_vec
        }
    }

    fn populate_flag_vector(args:&Vec<String>)->Vec<String>{
        let mut counter = 2;
        let mut res = vec![];
        while counter < args.len(){
            res.push(args[counter].clone());
            counter += 1;
        } 
        res
    }
}
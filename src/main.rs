use std::{env,process};
use gvmmman::*;

fn main(){
    let args: Vec<String> = env::args().collect();
    if let Err(e) = run(args){
        println!("Application error: {e}");
        process::exit(1);
    }
}
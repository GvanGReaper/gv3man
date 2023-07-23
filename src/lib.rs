pub mod app_state;
pub mod logger;
pub mod cmd_ui;
pub mod args;
pub mod checks;
pub mod commands;
pub mod mod_tracker;

use args::*;
use app_state::*;
use commands::*;
use mod_tracker::*;

use std::fs;
use std::{error::Error, vec};





fn setup(dirs: Vec<String>)->std::io::Result<()>{
    let dirs_to_create = dirs;
    for dir in dirs_to_create{
        fs::create_dir(dir)?;
    } 
    Ok(())
}





pub fn run(args: Vec<String>)-> Result<(),Box<dyn Error>>{

    
    let state_struct = AppState::initialize();
    let mod_tracker = ModTracker::initialize(&state_struct);
    // let logger = Logger::initialize(format!("{}/logs",state_struct.get_setup_dir_path()));
    // dbg!(logger);
    // println!("{state_struct}");
   
    //SETUP
    //---------------------------------------------------------------------------------
    match state_struct.get_missing_dirs(){
        Some(dirs)=>{
            setup(dirs)?;
        }
        None=>{}
    }   
    //---------------------------------------------------------------------------------


    let args_struct = Args::parse_args(args);
    handle_command(&args_struct,&state_struct);
    


    Ok(())
}



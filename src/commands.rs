use std::fs::{File,create_dir, remove_dir_all,copy};
use std::{io, process};
use std::path::Path;



use crate::cmd_ui::*;
use crate::{args::Args, app_state::AppState};
use crate::checks::*;
use crate::mod_tracker::*;

// pub fn recognize_command(command_str: &str)->bool{
//     let command_vec:Vec<&str> = vec!["show","help","add","list"];
//     for command in command_vec{
//         if command_str == command{
//             return true;
//         }
//     }
//     false
// }

pub fn handle_command(arg: &Args,state: &AppState){
    let command = &arg.command;
    let flags = &arg.flags;
    if command == "help"{
        show_help_message();
    }
    else if command == "list"{
        list(state,flags);
    }
    else if command == "new"{
        new(state,flags);
    }
    else if command == "delete"{
        delete(state,flags);
    }
    else if command == "empty"{
        empty(state,flags);
    }
    else{
        wrong_command(command);
    }
}




fn list(state: &AppState,flags: &Vec<String>){
    let parent_path = state.get_setup_dir_path();
    let pack_name;
    check_num_of_flags("list",flags,1,0);
    let final_path: String;
    if flags.len() == 0{
        final_path = format!("{}/mods",parent_path);
    }
    else{
        pack_name = flags[0].clone();
        final_path = format!("{}/mods/{}",parent_path,pack_name);
    }
    // dbg!(&final_path);
    check_if_dir_exists(&final_path);
    println!("----------------------------START----------------------------");
    for result in Path::new(&final_path).read_dir().expect("Should have been able to open the files"){
        match result{
            Ok(entry)=>{
                println!("{}",entry.file_name().to_str().unwrap())
            }
            Err(e)=>{
                panic!("ERROR: {}",e);
            }
        }
    }
    println!("-----------------------------END-----------------------------");
}

fn new(state: &AppState,flags: &Vec<String>){
    let parent_path = state.get_setup_dir_path();
    let pack_name:String;
    let path_to_copy_from: String;
    check_num_of_flags("list",flags,2,1);
    pack_name = flags[0].clone();    
    let final_path = format!("{}/mods/{}",parent_path,pack_name);
    if !&Path::new(&final_path).is_dir(){
        create_dir(Path::new(&final_path)).expect("ERROR:Should have been able to create dir");
        println!("Modpack with name {} created succesfully!",pack_name)
    }
    else{
        println!("Mod pack with name {} already exists,try giving command <list {}> to see its contents and <list> to see all existing modpacks",pack_name,pack_name);
        process::exit(1);
    }
    //IF (path_to_folder) is given:
    if flags.len() == 2{
        path_to_copy_from = flags[1].clone();
        for result in Path::new(&path_to_copy_from).read_dir().expect("Should have been able to open the files"){
            match result{
                Ok(entry)=>{
                    // println!("{}",entry.file_name().to_str().unwrap())
                    //current name and current name str exist cause borrow checker threw a fit.Do NOT try to change them
                    let current_name = entry.file_name();
                    let current_name_str = current_name.to_str().unwrap();
                    let file_to_make = format!("{}/{}",final_path,current_name_str);
                    let file_to_copy = format!("{}/{}",path_to_copy_from,current_name_str);
                    // dbg!(&file_to_copy);
                    let mut new_file = File::create(Path::new(&file_to_make)).expect("ERROR: should have been able to make new file");
                    let mut file_copy_from = File::open(Path::new(&file_to_copy)).expect("Should have been able to read file");
                    // dbg!(file);
                    let res = io::copy(&mut file_copy_from,&mut new_file);
                    match res{
                        Ok(bytes)=>{
                            dbg!(bytes);
                        }
                        Err(e)=>{
                            println!("ERROR: {}",e);
                            process::exit(1);
                        }
                    }
                }
                Err(e)=>{
                    panic!("ERROR: {}",e);
                }
            }
        }
    }
}

fn delete(state: &AppState,flags: &Vec<String>){
    let parent_path = state.get_setup_dir_path();
    let pack_name:String;
    check_num_of_flags("list",flags,1,1);
    pack_name = flags[0].clone();
    let final_path = format!("{}/mods/{}",parent_path,pack_name);
    if Path::new(&final_path).is_dir(){
        remove_dir_all(final_path).expect("ERROR: Should have been able to delete dir");        
        println!("Modpack with name {} deleted succesfully!",pack_name);
    }
    else{
        println!("Mod pack with name {} doesnt exists,try giving command <list> to see all existing modpacks",pack_name);
    }
}

fn empty(state: &AppState,flags: &Vec<String>){
    check_num_of_flags("empty",flags,1,1);     
    dbg!(flags);  
}
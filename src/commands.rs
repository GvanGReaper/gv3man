use std::fs::{File,create_dir, remove_dir_all,copy, read_dir, ReadDir, remove_file};
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
        dbg!(arg);
        show_command_list();
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
    else if command == "copy"{
        copy_command(state, flags);
    }
    else if command == "load"{
        load(state,flags);
    }
    else{
        wrong_command(command);
    }
}




fn list(state: &AppState,flags: &Vec<String>){
    let parent_path = state.get_setup_dir_path();
    let tlauncher_path = state.get_mc_mods_dir_path();
    // dbg!(&tlauncher_path);
    let pack_name;
    // check_num_of_flags("list",flags,1,0);
    // if flags.len() > 0{
    //     check_flags("list",flags,&vec!["-t","-d"]);   
    // }
    
    
    let final_path: String;
    if flags.len() == 0{
        final_path = format!("{}/mods",parent_path);
    }
    else if flags[0] == "-t"{
        final_path = format!("{}",tlauncher_path);
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
        if flags[1] == "-t"{
            path_to_copy_from = state.get_mc_mods_dir_path();
        }
        else{
            path_to_copy_from = flags[1].clone();
        }
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
                            // dbg!(bytes);
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
    // dbg!(flags);  
    let parent_path = state.get_setup_dir_path();
    let tlauncher_path = state.get_mc_mods_dir_path();
    let final_path;
    // dbg!(&parent_path);

    if flags[0] == "-t"{
        final_path = format!("{}",tlauncher_path);
    }
    else{
        let folder_name = flags[0].clone();
        final_path = format!("{}/mods/{}",parent_path,folder_name);
    }
    // dbg!(&final_path);
    
   
    let read_dir_res = read_dir(&final_path);
    let dir_content;
    match read_dir_res{
        Err(e)=>{
            println!("ERROR while trying to read directory at: {}\nError: {}",final_path,e);
        }
        Ok(content)=>{
            dir_content = content;
            for file in dir_content{
                match file {
                    Ok(entry)=>{
                        remove_file(entry.path()).expect("Should have been able to remove file");
                    }
                    Err(e)=>{
                        println!("ERROR while trying to read directory at: {}\nError: {}",final_path,e);
                    }
                }
            }
            // dbg!(dir_content);

        }
    }
}

fn load(state: &AppState,flags: &Vec<String>){
    check_num_of_flags("load",flags,1,1);
    let folder_name = flags[0].clone();
    let parent_path = state.get_setup_dir_path();
    copy_command(state, &vec![folder_name.clone(),"-t".to_string()]);
    println!("Loaded modpack {} succesfully!",folder_name);
}


fn copy_command(state: &AppState,flags: &Vec<String>){
    let parent_path = state.get_setup_dir_path();
    check_num_of_flags("copy",flags,3,2);
    let folder_to_copy;
    let folder_to_write;
    let final_copy_path;
    let final_write_path;
    let mut mode = "-w";

    
    if flags[0] == "-t"{
        folder_to_copy = state.get_mc_mods_dir_path();
        final_copy_path = folder_to_copy;
    }
    else{
        folder_to_copy = flags[0].clone();
        final_copy_path = format!("{}/mods/{}",parent_path,folder_to_copy);
    }
    if flags[1] == "-t"{
        folder_to_write = state.get_mc_mods_dir_path();
        final_write_path = folder_to_write.clone();
    }
    else{
        folder_to_write = flags[1].clone();
        final_write_path = format!("{}/mods/{}",parent_path,folder_to_write);
    }
    if flags.len() == 3{
        check_flags("copy",&vec![flags[2].clone()],&vec!["-w","-a","-e"]);
        mode = &flags[2];
    }   
    empty(state,&vec![folder_to_write.to_string().clone()]);
    // dbg!(&final_copy_path,&final_write_path,&mode);
    
    for result in Path::new(&final_copy_path).read_dir().expect("ERROR: Couldnt read directory to copy files from"){
        match result{
            Err(e)=>{
                println!("ERROR: failed to open and read contents of directory at: {},\nERROR description: {}",final_copy_path,e);
                process::exit(1);
            }
            Ok(entry)=>{
                let current_name = entry.file_name();
                let current_name_str = current_name.to_str().unwrap();
                let file_to_make = format!("{}/{}",final_write_path,current_name_str);
                let file_to_copy = format!("{}/{}",final_copy_path,current_name_str);
                // dbg!(&file_to_copy);
                let mut new_file = File::create(Path::new(&file_to_make)).expect("ERROR: should have been able to make new file");
                let mut file_copy_from = File::open(Path::new(&file_to_copy)).expect("Should have been able to read file");
                // dbg!(file);
                let res = io::copy(&mut file_copy_from,&mut new_file);
                match res{
                    Ok(bytes)=>{
                        //dbg!(bytes);
                    }
                    Err(e)=>{
                        println!("ERROR: {}",e);
                        process::exit(1);
                    }
                }
            }
        }
    }

}
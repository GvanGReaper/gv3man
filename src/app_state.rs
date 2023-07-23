use std::fs::{File,write};
use std::{env, fs};
use std::env::{consts::OS,*};
use std::path::{Path};
use std::fmt::Display;

use chrono::format::format;
use home::home_dir;

#[derive(Clone)]
pub struct AppState{
    os: String,
    working_dir_path: String,
    setup_dir_path: String,
    usr_config_path: String,
    mc_mods_dir_path: String
}



impl AppState{
    pub fn initialize()->AppState{     
        AppState { 
            os: OS.to_string(),
            working_dir_path:Self::initialize_current_dir(),
            setup_dir_path:Self::intialize_setup_dir(),
            usr_config_path:Self::initialize_usr_config_dir(),
            mc_mods_dir_path:Self::initialize_mods_dir(),
        }
    }

    pub fn get_os(&self)->String{
        self.os.clone()
    }

    pub fn get_working_dir_path(&self)->String{
        self.working_dir_path.clone()
    }

    pub fn get_setup_dir_path(&self)->String{
        self.setup_dir_path.clone()
    }

    pub fn get_usr_config_path(&self)->String{
        self.usr_config_path.clone()
    }

    pub fn get_mc_mods_dir_path(&self)->String{
        self.mc_mods_dir_path.clone()
    }

    fn intialize_setup_dir()->String{
        let path:&Path;
        let final_path: String;
        let home_path: String = home_dir().unwrap().into_os_string().into_string().unwrap();
        // dbg!(&home_path);
        if OS == "linux"{
            final_path = format!("{}/.gv3man",home_path);
            // dbg!(&final_path);
        }else{
            todo!()
        }
        // dbg!(path.is_dir());
        return final_path    
    }

    fn initialize_usr_config_dir()->String{
        let final_path:String;
        if OS == "linux"{
            final_path = String::from("/etc/gv3man");
        }
        else{
            todo!()
        }
        final_path
    }

    fn initialize_current_dir()->String{
        env::current_dir().unwrap().into_os_string().into_string().unwrap()
    }

    fn initialize_mods_dir()->String{
        let home_path: String = home_dir().unwrap().into_os_string().into_string().unwrap();
        let mut final_path: String = String::new();
        if OS == "linux"{
            final_path = format!("{}/.minecraft/mods",home_path)
        }
        else{
            todo!()
        }
        final_path
    }

    pub fn get_missing_dirs(&self)->Option<Vec<String>>{
        let mut missing_dir_paths: Vec<String> = vec![];
        //CHECK FOR SETUP PATH
        if !Path::new(&self.get_setup_dir_path()).is_dir(){
            missing_dir_paths.push(self.get_setup_dir_path().clone());
            //ALSO ADD THE LOG AND MOD SUBFOLDERS
            let parent_path = self.get_setup_dir_path().clone();
            let log_dir = format!("{}/logs",parent_path);
            let mods_dir = format!("{}/mods",parent_path);
            missing_dir_paths.push(log_dir);
            missing_dir_paths.push(mods_dir);
        }
        //IF MAIN FOLDER ALREADY EXISTS,WE STILL CHECK FOR SUBFOLDERS INSIDE IT
        else{
            let parent_path = self.get_setup_dir_path().clone();
            let log_dir = format!("{}/logs",parent_path);
            let mods_dir = format!("{}/mods",parent_path);
            let mod_tracker_file = format!("{}/mod_tracker.gv3md",parent_path);
            // dbg!(mod_tracker_file);
            let temp = File::open(&mod_tracker_file);
            match temp{
                Err(e)=>{
                    write(&mod_tracker_file,"<DATA>\nmods: 0\nversions:\n<MODS>\n").expect("ERROR:Should have been able to create mod tracker file");
                }
                Ok(_)=>{}
            }
            if !Path::new(&log_dir).is_dir(){
                missing_dir_paths.push(log_dir)
            }
            if !Path::new(&mods_dir).is_dir(){
                missing_dir_paths.push(mods_dir)
            }
            
        }        
        //CHECK FOR USR_CONF PATH
        if !Path::new(&self.get_usr_config_path()).is_dir(){
            missing_dir_paths.push(self.get_usr_config_path().clone())
        }
        // dbg!(&missing_dir_paths);
        if missing_dir_paths.len() == 0{
            return None
        }
        else{
            return Some(missing_dir_paths)
        }
    }


    fn check_if_already_setup()->bool{
        let path:&Path;
        let final_path: String;
        let home_path: String = home_dir().unwrap().into_os_string().into_string().unwrap();
        // dbg!(&home_path);
        if OS == "linux"{
            final_path = format!("{}/.gvm3an",home_path);
            // dbg!(&final_path);
            path = Path::new(final_path.as_str());
        }else{
            todo!()
        }
        // dbg!(path.is_dir());
        return path.is_dir()
    }

}


impl Display for AppState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let working_dir = self.get_working_dir_path();
        let setup_dir = self.get_setup_dir_path();
        let user_config = self.get_usr_config_path();
        let mod_dir = self.get_mc_mods_dir_path();
        write!(f,"OS: {}\nCURRENT_WORKING_DIR: {}\nSETUP_DIR: {}\nMC_MOD_LOCATION: {}\nUSR_CONFIG_LOCATION: {}",OS,working_dir,setup_dir,mod_dir,user_config)
    }
}
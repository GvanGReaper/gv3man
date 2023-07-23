use crate::AppState;
use std::{fs::read_to_string, process};

#[derive(Debug)]
pub struct ModTracker{
    path: String,
    num_of_mods: u8,
    available_versions: Vec<String>
}

impl ModTracker{
    pub fn initialize(state: &AppState)->ModTracker{
        let file_path = format!("{}/mod_tracker.gv3md",state.get_setup_dir_path());
        let mod_num:u8;
        let version_vec: Vec<String>;
        (mod_num,version_vec) = Self::initialize_vals(&file_path);
        ModTracker {
            path: file_path,
            num_of_mods:mod_num, 
            available_versions:version_vec 
        }
    }   

    fn get_file_lines(path: &str)->Vec<String>{
        let mut res:Vec<String> = vec![];
        let temp = read_to_string(path);
        match temp{
            Ok(data)=>{
                let lines:Vec<&str> = data.lines().collect();
                res = lines.into_iter().map(|s| s.to_string()).collect();
            }
            Err(e)=>{
                println!("ERROR WHEN ATTEMPTING TO READ mod_tracker.gv3md: {}",e);
                process::exit(1);
            }
        }
        dbg!(&res);
        res
    }

    fn initialize_vals(path: &str)->(u8,Vec<String>){
        let res = read_to_string(path);
        let content:String;
        let mod_num:u8;
        let mut version_vec: Vec<String> = vec![];
        match res{
            Ok(data)=>{
                let lines:Vec<&str> = data.lines().collect();
                let mut temp:Vec<&str> = lines[1].split(":").collect();
                mod_num = temp[1].parse().expect("ERROR: VALUE GIVEN FOR MOD_NUM IN <DATA> AT mod_tracker.gv3md SHOULD have been parsable");
                // dbg!(mod_num);
                temp = lines[2].split(":").collect();
                Self::fill_version_vector(&mut version_vec,temp[1]);
            } 
            Err(e)=>{
                println!("ERROR WHEN ATTEMPTING TO READ mod_tracker.gv3md: {}",e);
                process::exit(1);
            }
        }
        (mod_num,version_vec)
    }

    fn fill_version_vector(version_vec: &mut Vec<String>,str_to_parse: &str){
        if str_to_parse != ""{
            let str_vec:Vec<&str> = str_to_parse.split(",").collect();
            for version in str_vec{
                version_vec.push(version.to_string());
            }
        }
    }

    fn check_if_already_exists(){

    }

    pub fn add_mod(){

    }

    pub fn remove_mod(){

    }

    pub fn get_num_of_mods(&self)->u8{
        self.num_of_mods
    }

    pub fn get_versions_vec(&self)->&Vec<String>{
        &self.available_versions
    }
}
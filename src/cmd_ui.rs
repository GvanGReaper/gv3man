use std::process;

use crate::args::Args;

fn show_default_message(){
    println!("Please give command: )")
}

pub fn show_help_message(){
    println!("LISTING ALL AVAILABLE COMMANDS:");
    println!("[] are required arguments,() are the optional ones");
    println!("------------------GENERAL:------------------");
    println!("help: shows help");
    println!("list (mod_pack_name): Lists all mod files in the mod folder with mod_pack_name,if there is no mod pack name it just lists all modpacks");
    println!("---------------EDIT MOD LISTS:--------------");
    println!("add [mod_name] [mod_pack_name]: Adds mod with mod_name to folder with mod_pack_name");
    println!("remove [mod_name] [mod_pack_name]: Removes mod with mod_name from folder with mod_pack_name");
    println!("delete [mod_pack_name]: Deletes mod_pack_name directory");
    println!("empty [mod_pack_name]: Empty mod_pack_name's dir of all mods");
    println!("new [mod_pack_name] (path_to_folder): Creates new empty modpack folder with mod_pack_name,if path_to_folder is give it copies the contents of that folder instead");
    println!("copy [mod_pack_name] [target_name] (mode): Copies the contents of target_name to mod_pack_name.Will create [mod_pack_name] by default if it doesnt exist.Flag has three modes:");
    println!("    -w: Completely override existing content(default mode).");
    println!("    -a: Append to alredy existing content.");
    println!("    -e: Error when file doesnt exist instead of creating it.");
    println!("-------------MANAGE MOD LISTS:--------------");
    println!("activate [mod_pack_name]: Puts mod_pack_name in tlauncher's mods folder");
    println!("clear: Clears tlauncher's mods folder");
    println!("--------------------------------------------");

}

pub fn wrong_command(command: &str){
    println!("{} is not a recognized command!",command);
    println!("Did you perhaps make a spelling mistake?");
    println!("To see available commands,give command <help>");
    process::exit(1);
}

pub fn missing_argument(arg_name: &str){
    println!("Insufficient arguments given.Missing {} argument",arg_name);
    process::exit(1);
}


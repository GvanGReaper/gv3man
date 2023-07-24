use std::process;

use crate::args::Args;

fn show_default_message(){
    println!("Please give command: )")
}

pub fn show_help_message(){
    println!("LISTING ALL AVAILABLE COMMANDS:");
    println!("[] are required arguments,() are the optional ones");
    println!("| indicates that only one of the two arguments can be given at a time");
    println!("------------------GENERAL:------------------");
    println!("help: shows help");
    println!("list (dir_name)|(flag): Lists all mod pack folders.If no suitable flag mode is given,it will list mods in the dir_name folder instead.Flag has one mode:");
    println!("    -t: Shows tlauncher's mods folder contents.");
    println!("---------------EDIT MOD LISTS:--------------");
    println!("add [mod_name] [mod_pack_name]: Adds mod with mod_name to folder with mod_pack_name");
    println!("remove [mod_name] [mod_pack_name]: Removes mod with mod_name from folder with mod_pack_name");
    println!("delete [mod_pack_name]: Deletes mod_pack_name directory");
    println!("empty [mod_pack_name]|(flag): Checks for flags,If no suitable flag arg is given then it will empty the folder with mod_pack_name.Flag modes are:");
    println!("    -t: Empties tlauncher's mod folder contents.");
    println!("new [mod_pack_name] (path_to_folder)|(flag): Creates new empty modpack folder with mod_pack_name,if path_to_folder is give it copies the contents of that folder instead.If flag is given,modes are:");
    println!("    -t: Creates the new folder with the content being a copy of the content that is in the moment stored in tlauncher's mod folder.");
    println!("copy [mod_pack_name] [target_name] (mode): Copies the contents of target_name to mod_pack_name.Will create [mod_pack_name] by default if it doesnt exist.Flag has three modes:");
    println!("    -w: Completely override existing content(default mode).");
    println!("    -a: Append to alredy existing content.");
    println!("    -e: Error when file doesnt exist instead of creating it.");
    println!("-------------MANAGE MOD LISTS:--------------");
    println!("activate [mod_pack_name]: Puts mod_pack_name in tlauncher's mods folder");
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


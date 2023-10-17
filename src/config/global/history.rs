use std::{fs::*, path::Path};

pub fn check_history_file()-> bool{

    let file_path= "/home/.phoenix";
    let path = Path::new(file_path);

    if path.exists() {
        true
    }
    else{
        false
    }
}
pub fn read_history_file(){

}

pub fn create_history_file(){
    
}

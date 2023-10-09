use nix::unistd::Uid;

pub fn check_admin()-> bool{
    let is_root :bool;
    if Uid::effective().is_root() == true{
        is_root = true;
    }
    else{
        is_root = false;
    }
    return is_root;
}
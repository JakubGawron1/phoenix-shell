mod config;
mod interface;
fn main() {
    let mut  os_current_dir: String = r"C\Windows\".to_string();
    if cfg!(unix){
        os_current_dir = "/home/".to_string();
    }
    let os_current_dir = os_current_dir.into(); 

    let mut new_interface = interface::Interface{
        current_dir: os_current_dir,
        is_admin: config::admin::check_admin(),
    };
    interface::Interface::init(&mut new_interface);
    
}

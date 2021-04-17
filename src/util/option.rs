use super::sys;
use std::time::Duration;

pub struct UriOption{
    pub base_uri: String,
    pub use_wordlist: bool,
    pub wordlist_path: String,
    pub request_method: String,
    pub timeout: Duration,
    pub save_path: String,
}

pub struct DomainOption{
    pub base_domain: String,
    pub use_wordlist: bool,
    pub wordlist_path: String,
    pub timeout: Duration,
    pub save_path: String,
}

impl UriOption {
    pub fn new() -> UriOption {
        let uri_option = UriOption {
            base_uri: String::new(),
            use_wordlist: false,
            wordlist_path: String::new(),
            request_method: String::new(),
            timeout: Duration::from_millis(30000),
            save_path: String::new(),
        };
        return uri_option;
    }
    pub fn set_option(&mut self, arg_value: String){
        if arg_value.ends_with("/") {
            self.base_uri = arg_value;
        }else{
            self.base_uri = format!("{}/", arg_value);
        }
    }
    pub fn set_file_path(&mut self, file_path: String){
        if !file_path.is_empty() {
            self.use_wordlist = true;
            self.wordlist_path = file_path;   
        }
    }
    pub fn set_request_method(&mut self, method_name: String){
        self.request_method = method_name;
    }
    pub fn set_timeout(&mut self, ms_str: String){
        let timeout: u64 = ms_str.parse().unwrap();
        self.timeout = Duration::from_millis(timeout);
    }
    pub fn set_save_path(&mut self, save_path: String){
        self.save_path = save_path;
    }
    pub fn show_options(&self){
        sys::print_fix32("URI Scan Options", sys::FillStr::Hyphen);
        println!("{}Base URI: {}", sys::SPACE4, self.base_uri);
        if self.use_wordlist {
            println!("{}Word list: {}", sys::SPACE4, self.wordlist_path);
        }
        sys::print_fix32("", sys::FillStr::Hyphen);
    }
}

impl DomainOption {
    pub fn new() -> DomainOption {
        let domain_option = DomainOption {
            base_domain: String::new(),
            use_wordlist: false,
            wordlist_path: String::new(),
            timeout: Duration::from_millis(30000),
            save_path: String::new(),
        };
        return domain_option;
    }
    pub fn set_option(&mut self, arg_value: String){
        self.base_domain = arg_value;
    }
    pub fn set_file_path(&mut self, file_path: String){
        if !file_path.is_empty() {
            self.use_wordlist = true;
            self.wordlist_path = file_path;   
        }
    }
    pub fn set_timeout(&mut self, ms_str: String){
        let timeout: u64 = ms_str.parse().unwrap();
        self.timeout = Duration::from_millis(timeout);
    }
    pub fn set_save_path(&mut self, save_path: String){
        self.save_path = save_path;
    }
    pub fn show_options(&self){
        sys::print_fix32("Domain Scan Options", sys::FillStr::Hyphen);
        println!("{}Base Domain Name: {}", sys::SPACE4, self.base_domain);
        if self.use_wordlist {
            println!("{}Word list: {}", sys::SPACE4, self.wordlist_path);
        }
        sys::print_fix32("", sys::FillStr::Hyphen);
    }
}

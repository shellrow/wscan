#[macro_use]
extern crate clap;

mod util;

use clap::{App, AppSettings, Arg, ArgGroup};
use std::io::{stdout, Write};
use std::env;
use std::fs::read_to_string;
use chrono::{Local, DateTime};
use util::{option, validator, sys};
use util::sys::{SPACE4};
use dns_lookup::lookup_host;
use crossterm::style::Colorize;
use webscan::{RequestMethod, ScanStatus};
use webscan::{UriScanner, DomainScanner};
use webscan::{UriScanResult, DomainScanResult};

const CRATE_UPDATE_DATE: &str = "2021/4/18";
const CRATE_AUTHOR_GITHUB: &str = "shellrow <https://github.com/shellrow>";

#[cfg(target_os = "windows")]
fn get_os_type() -> String{"windows".to_owned()}

#[cfg(target_os = "linux")]
fn get_os_type() -> String{"linux".to_owned()}

#[cfg(target_os = "macos")]
fn get_os_type() -> String{"macos".to_owned()}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        show_app_desc();
        std::process::exit(0);
    }
    let app = get_app_settings();
    let matches = app.get_matches();
    show_banner_with_starttime();
    if matches.is_present("uri"){
        if let Some(v) = matches.value_of("uri") {
            let mut opt = option::UriOption::new();
            opt.set_option(v.to_string());
            if let Some(w) = matches.value_of("word") {
                opt.set_file_path(w.to_string());
            }
            if let Some(m) = matches.value_of("method") {
                opt.set_request_method(m.to_string());
            }
            if let Some(t) = matches.value_of("timeout") {
                opt.set_timeout(t.to_string());
            }
            if let Some(s) = matches.value_of("save") {
                opt.set_save_path(s.to_string());
            }
            handle_uri_scan(opt).await;
        }
    }else if matches.is_present("domain"){
        if let Some(v) = matches.value_of("domain") {
            let mut opt = option::DomainOption::new();
            opt.set_option(v.to_string());
            if let Some(w) = matches.value_of("word") {
                opt.set_file_path(w.to_string());
            }
            if let Some(t) = matches.value_of("timeout") {
                opt.set_timeout(t.to_string());
            }
            if let Some(s) = matches.value_of("save") {
                opt.set_save_path(s.to_string());
            }
            handle_domain_scan(opt).await;
        }
    }else{
        println!();
        println!("Error: Scan mode not specified.");
        std::process::exit(0);
    }
}

fn get_app_settings<'a, 'b>() -> App<'a, 'b> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(CRATE_AUTHOR_GITHUB)
        .about(crate_description!())
        .arg(Arg::with_name("uri")
            .help("URI Scan - Ex: -u http://192.168.1.8/xvwa/ -w common.txt")
            .short("u")
            .long("uri")
            .takes_value(true)
            .validator(validator::validate_uri_opt)
        )
        .arg(Arg::with_name("domain")
            .help("Domain Scan - Ex: -d example.com -w subdomain.txt")
            .short("d")
            .long("domain")
            .takes_value(true)
            .value_name("domain_name")
            .validator(validator::validate_domain_opt)
        )
        .arg(Arg::with_name("timeout")
            .help("Set timeout in ms - Ex: -t 10000")
            .short("t")
            .long("timeout")
            .takes_value(true)
            .value_name("duration")
            .validator(validator::validate_timeout)
        )
        .arg(Arg::with_name("word")
            .help("Use word list - Ex: -w common.txt")
            .short("w")
            .long("word")
            .takes_value(true)
            .value_name("file_path")
            .validator(validator::validate_filepath)
        )
        .arg(Arg::with_name("method")
            .help("Set HTTP request method for scanning")
            .short("m")
            .long("method")
            .takes_value(true)
            .value_name("method")
            .validator(validator::validate_request_method)
        )
        .arg(Arg::with_name("save")
            .help("Save scan result to file - Ex: -s result.txt")
            .short("s")
            .long("save")
            .takes_value(true)
            .value_name("file_path")
        )
        .group(ArgGroup::with_name("mode")
            .args(&["uri", "domain"])
        )
        .setting(AppSettings::DeriveDisplayOrder)
        ;
        app
}

fn show_app_desc() {
    println!("{} {} ({}) {}", crate_name!(), crate_version!(), CRATE_UPDATE_DATE, get_os_type());
    println!("{}", crate_description!());
    println!("{}", CRATE_AUTHOR_GITHUB);
    println!();
    println!("'{} --help' for more information.", crate_name!());
    println!();
}

fn show_banner_with_starttime() {
    println!("{} {} {}", crate_name!(), crate_version!(), get_os_type());
    println!();
    let local_datetime: DateTime<Local> = Local::now();
    println!("Scan started at {}", local_datetime);
    println!();
}

async fn handle_uri_scan(opt: option::UriOption) {
    opt.show_options();
    println!();
    print!("Scanning...");
    stdout().flush().unwrap();
    let mut uri_scanner = match UriScanner::new(){
        Ok(scanner) => (scanner),
        Err(e) => panic!("Error creating scanner: {}", e),
    };
    uri_scanner.set_base_uri(opt.base_uri.to_string());
    if opt.use_wordlist {
        let data = read_to_string(opt.wordlist_path.to_string());
        let text = match data {
            Ok(content) => content,
            Err(e) => {panic!("Could not open or find file: {}", e);}
        };
        let word_list: Vec<&str> = text.trim().split("\n").collect();
        for word in word_list {
            uri_scanner.add_word(word.to_string());
        }
    }
    if !opt.request_method.is_empty() {
        if opt.request_method.to_uppercase() == "POST" {
            uri_scanner.set_request_method(RequestMethod::Post);
        }else{
            uri_scanner.set_request_method(RequestMethod::Get);
        }
    }
    uri_scanner.set_timeout(opt.timeout);
    uri_scanner.run_scan().await;
    let result = uri_scanner.get_result();
    match result.scan_status {
        ScanStatus::Done => {println!("{}", "Done".green())},
        ScanStatus::Timeout => {println!("{}", "Timed out".yellow())},
        _ => {println!("{}", "Error".red())},
    }
    println!();
    sys::print_fix32("Scan Reports", sys::FillStr::Hyphen);
    for (uri, status) in result.responses {
        if status.starts_with("2") {
            println!("{}{} {}", SPACE4, uri, status.green());
        }else if status.starts_with("4") {
            println!("{}{} {}", SPACE4, uri, status.red());
        }else if status.starts_with("5") {
            println!("{}{} {}", SPACE4, uri, status.red());
        }else{
            println!("{}{} {}", SPACE4, uri, status);
        }
    }
    sys::print_fix32("", sys::FillStr::Hyphen);
    println!("Scan Time: {:?}", result.scan_time);
    if !opt.save_path.is_empty() {
        let s_result = uri_scanner.get_result();
        save_uri_result(&opt, s_result);
    }
}

async fn handle_domain_scan(opt: option::DomainOption) {
    opt.show_options();
    println!();
    print!("Scanning...");
    stdout().flush().unwrap();
    let mut domain_scanner = match DomainScanner::new(){
        Ok(scanner) => (scanner),
        Err(e) => panic!("Error creating scanner: {}", e),
    };
    domain_scanner.set_base_domain(opt.base_domain.to_string());
    if opt.use_wordlist {
        let data = read_to_string(opt.wordlist_path.to_string());
        let text = match data {
            Ok(content) => content,
            Err(e) => {panic!("Could not open or find file: {}", e);}
        };
        let word_list: Vec<&str> = text.trim().split("\n").collect();
        for d in word_list{
            domain_scanner.add_word(d.to_string());
        }
    }
    domain_scanner.set_timeout(opt.timeout);
    domain_scanner.run_scan().await;
    let result = domain_scanner.get_result();
    match result.scan_status {
        ScanStatus::Done => {println!("{}", "Done".green())},
        ScanStatus::Timeout => {println!("{}", "Timed out".yellow())},
        _ => {println!("{}", "Error".red())},
    }
    println!();
    sys::print_fix32("Scan Reports", sys::FillStr::Hyphen);
    println!("{}", opt.base_domain.to_string());
    match lookup_host(&opt.base_domain){
        Ok(ips) => {
            for ip in ips{
                println!("{}{}",SPACE4, ip);
            }
        },
        Err(e) => {println!("{} {}", e, opt.base_domain);},
    }
    println!();
    for (domain, ips) in result.domain_map {
        println!("{}{}", SPACE4.repeat(2), domain);
        for ip in ips{
            println!("{}{}", SPACE4.repeat(3), ip);
        }
    }
    sys::print_fix32("", sys::FillStr::Hyphen);
    println!("Scan Time: {:?}", result.scan_time);
    if !opt.save_path.is_empty() {
        let s_result = domain_scanner.get_result();
        save_domain_result(&opt, s_result);
    }
}

fn save_uri_result(opt: &option::UriOption, result: UriScanResult){
    let mut data = "[OPTIONS]".to_string();
    data = format!("{}\nBASE_URI: {}",data, opt.base_uri.to_string());
    data = format!("{}\nWORD_LIST: {}",data, opt.wordlist_path.to_string());
    data = format!("{}\n[RESULTS]",data);
    for (uri, status) in result.responses {
        data = format!("{}\n{},{}",data,uri,status);
    }
    data = format!("{}\n",data);
    sys::save_file(opt.save_path.to_string(), data);
}

fn save_domain_result(opt: &option::DomainOption, result: DomainScanResult){
    let mut data = "[OPTIONS]".to_string();
    data = format!("{}\nBASE_DOMAIN: {}",data, opt.base_domain.to_string());
    data = format!("{}\nWORD_LIST: {}",data, opt.wordlist_path.to_string());
    data = format!("{}\n[RESULTS]",data);
    match lookup_host(&opt.base_domain){
        Ok(ips) => {
            data = format!("{}\n{}", data, opt.base_domain.to_string());
            for ip in ips{
                data = format!("{},{}", data, ip);
            }
        },
        Err(_) => {},
    }
    for (domain, ips) in result.domain_map {
        data = format!("{}\n{}",data,domain);
        for ip in ips{
            data = format!("{},{}",data,ip);
        }
    }
    data = format!("{}\n",data);
    sys::save_file(opt.save_path.to_string(), data);
}

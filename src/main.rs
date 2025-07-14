// <-----------> Importing standard libraries <----------->

static mut DISPLAY_DEBUG_INFO: bool = false;
static mut OVERRIDE_EXISTING_FILES: bool = false;
//static mut CURRENT_DIRECTORY: &str = "Download/";
//mod app; // deprecated
mod app_iced;
mod app_iced_style;
mod crawler;
mod pdf_maker;
// std(s)
use std::env;
//use std::fs;
//use std::fs::File;
use std::io;
//use std::io::prelude::*;
//use std::io::BufReader;
//use std::path::Path;
//use std::thread;
// use(s)
#[cfg(windows)]
use colored::control;
use colored::Colorize;
#[cfg(windows)]
use crossterm;

//use colored::*;
//use gtk::prelude::*;
//use gtk::glib;
//use gtk4 as gtk;
//use gtk4::cairo::ffi::STATUS_SUCCESS;
//use reqwest::Client;
//use scraper::ElementRef;
//use scraper::{Html, Selector};
macro_rules! logln {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            "[Main]".bold().green(),
            format!($($arg)*)
        );
    };
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_ansi_support();
    println!(
        "{} {}",
        "[Main]".bold().green(),
        "******* PASS IN `--cli` to use cli *******"
            .bold()
            .bright_purple()
            .underline()
    );

    let mut args: Vec<String> = env::args().collect();

    // COMMAND LINE ARGS
    if args.len() <= 1 {
        println!("[{}] NO COMMANDS PASSED IN", "Main".green().bold());
        args.push("--gui".to_string());
        args.push("".to_string());
    }

    if args.contains(&"--cli".to_string()) || args.contains(&"-c".to_string()) {
        logln!("Running cli");
    } else if args.contains(&"--gui".to_string()) || args.contains(&"-g".to_string()) {
        logln!("Running gui");
        let result: iced::Result = app_iced::create_application();
        match result {
            Ok(()) => {
                println!("[{}] GUI exited without any error.", "Main".green().bold());
                std::process::exit(0);
            }
            Err(e) => {
                panic!("[{}] GUI exited with error: {}", "Main".green().bold(), e);
            }
        }
    }
    if args.contains(&"--debug".to_string()) || args.contains(&"-d".to_string()) {
        logln!("Enabling debug mode....");
        unsafe {
            DISPLAY_DEBUG_INFO = true;
        }
    }
    if args.contains(&"--ovveride".to_string()) || args.contains(&"-o".to_string()) {
        logln!("Will override existing files...");
        unsafe {
            OVERRIDE_EXISTING_FILES = true;
        }
    }
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        logln!(
            "--help used:
            --cli or -c                     Runs cli
            --gui or -g                     Runs gui(Default: Runs gui instead of cli.) 
            --help or -h                    Shows this help form
            --url <url> or -u <url>         Used to set the url
            --debug or -d                   Displays debug info
            --overrride or -o               Overrides the pre-existing files with the new one (Default: Skips re-downloading the pre-existing files.)
            -----------------------------------------------------------------------------------------------------------------------------------------"
        );
    }

    // control::set_virtual_terminal(true).unwrap();
    // crossterm::terminal::enable_virtual_terminal_processing(std::io::stdout()).unwrap();
    println!(
        "{}",
        "******* Downloader Started *******"
            .bold()
            .bright_cyan()
            .underline()
    );

    println!(
        "Download PDFs(Enter {} for yes or {} for no):",
        "y".bold().green(),
        "n".bold().red()
    );
    let mut download_pdfs: String = String::new();
    io::stdin()
        .read_line(&mut download_pdfs)
        .expect("failed to readline");
    let download_pdfs = download_pdfs.trim().chars().next().unwrap();

    println!(
        "Download Images(Enter {} for yes or {} for no):",
        "y".bold().green(),
        "n".bold().red()
    );
    let mut download_imgs: String = String::new();
    io::stdin()
        .read_line(&mut download_imgs)
        .expect("failed to readline");
    let download_imgs = download_imgs.trim().chars().next().unwrap();

    println!(
        "Scan All Subfolders(Enter {} for yes or {} for no):",
        "y".bold().green(),
        "n".bold().red()
    );
    let mut scan_subfolders: String = String::new();
    io::stdin()
        .read_line(&mut scan_subfolders)
        .expect("failed to readline");
    let scan_subfolders = scan_subfolders.trim().chars().next().unwrap();

    if let Some(index) = args.iter().position(|s| s == "--url" || s == "-u") {
        logln!("URL PASSED IN");

        if let Some(url) = args.get(index + 1) {
            logln!("Url is: {}", url);
            let _ = crawler::get_table(
                url.as_str(),
                "Download/",
                download_pdfs,
                download_imgs,
                scan_subfolders,
            )
            .await;
        } else {
            logln!("ERROR: url not passed in read --help");
        }
    } else {
        logln!("Enter the url: ");

        let mut url: String = String::new();

        io::stdin().read_line(&mut url).expect("failed to readline");
        let _ = crawler::get_table(
            url.as_str(),
            "Download/",
            download_pdfs,
            download_imgs,
            scan_subfolders,
        )
        .await;
    }

    println!(
        "{}",
        "******* Task Completed! *******".bold().underline().red()
    );
    println!(
        "Convert jpegs to pdf?(Enter {} for yes or {} for no):",
        "y".bold().green(),
        "n".bold().red()
    );

    let mut choice: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("failed to readline");

    let choice = choice.trim().chars().next().unwrap();

    match choice {
        'y' => {
            println!("[{}] converting...", "Main".green().bold());
            let _ = pdf_maker::convert_jpegs_to_pdf();
        }
        'n' => {
            println!("[{}] not converting", "Main".green().bold());
        }
        _ => {
            println!("[{}] Error reading input", "Main".green().bold());
        }
    }
    Ok(()) // Return statement
}

#[cfg(windows)]
fn enable_ansi_support() {
    println!("[Main] Detected Windows.... Enabling ANSI SUPPORT for colors...");
    control::set_virtual_terminal(true).unwrap();
    // crossterm::terminal::enable_virtual_terminal_processing(std::io::stdout()).unwrap();
}

#[cfg(not(windows))]
fn enable_ansi_support() {
    println!("[{}] Detected UNIX BASED OS....", "Main".green().bold());
}

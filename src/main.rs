// <-----------> Importing standard libraries <----------->

//static mut CURRENT_DIRECTORY: &str = "Download/";
//mod app; // deprecated
mod app_iced;
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
use colored::Colorize;
#[cfg(windows)]
use colored::control;
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

    if args.len() <= 1 {
        println!("[{}] NO COMMANDS PASSED IN", "Main".green().bold());
        args.push("gui".to_string());
        args.push("".to_string());
    }
    let query = &args[1];
    match query.as_str() {
        "--cli" => {
            println!("[{}] RUNNING CLI", "Main".green().bold())
        }
        _ => {
            println!("[{}] RUNNING GUI", "Main".green().bold());
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

            //let result = app::create_application().await;
            //match result {
            //    glib::ExitCode::SUCCESS => {
            //        panic!("_ UI CLOSED");
            //    }
            //    _ => {
            //        panic!("GUI CRASHED :(");
            //    }
            //}
        }
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

    let url = &args[2];
    match url.as_str() {
        "" => {
            println!("[{}] URL not passed in", "Main".green().bold());
            let _ = crawler::read_urls_from_file(download_pdfs, download_imgs, scan_subfolders);
        }
        _ => {
            println!("[{}] URL: {}", "Main".green().bold(), url.red());
            let _ = crawler::get_table(
                url.as_str(),
                "Download/",
                download_pdfs,
                download_imgs,
                scan_subfolders,
            )
            .await;
        }
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
    crossterm::terminal::enable_virtual_terminal_processing(std::io::stdout()).unwrap();
}

#[cfg(not(windows))]
fn enable_ansi_support() {
    println!("[{}] Detected UNIX BASED OS....", "Main".green().bold());
}

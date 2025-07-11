// <-----------> Importing standard libraries <-----------
#![allow(static_mut_refs)]
static mut CURRENT_DIRECTORY: &str = "Download/";
// std(static)
//use std::env;
use std::fs;
use std::fs::File;
//use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
//use std::thread;
// use(s)
use colored::Colorize;
//use colored::*;
//use gtk::prelude::*;
//use gtk::{
//    glib, Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, Orientation,
//};
//use gtk4 as gtk;
//use gtk4::cairo::ffi::STATUS_SUCCESS;

use reqwest::Client;
use scraper::ElementRef;
use scraper::{Html, Selector};

pub async fn get_table(
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("{} GETTING TABLE", "[Crawler]".bold().red());
    println!("{} URL : {}", "[Crawler]".bold().red(), url.bold().blink()); // Print the URL

    let response = ureq::get(url).call()?; // send a request to the url
                                           // let html: Html = Html::parse_document(&response.into_string()?); // parse the html from the response
    let html = Box::leak(Box::new(Html::parse_document(&response.into_string()?)));
    let _ = extract_table(
        url,
        file_path,
        download_pdfs,
        download_imgs,
        scan_subfolders,
        html,
    )
    .await;
    // Get all tables from html

    Ok(None) // return None
}

pub async fn extract_table(
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
    html: &'static Html,
) {
    println!("{} EXTRACTING TABLE", "[Crawler]".bold().red());

    let table_selector = Selector::parse("table").unwrap(); // make table selector
    for table in html.select(&table_selector) {
        // get all tables from html

        let row_selector = Selector::parse("tr").unwrap(); // make a row selector

        // Get all rows from table
        for row in table.select(&row_selector) {
            // get all rows from table

            let href_selector = Selector::parse("a[href]").unwrap(); // make a href selector

            for href in row.select(&href_selector) {
                // Get all links from row

                let _href_attr = href.value().attr("href").unwrap(); // gets the href attribute

                let img_selector = Selector::parse("img").unwrap();

                for img in row.select(&img_selector) {
                    let _ = get_images(
                        img,
                        href,
                        url,
                        file_path,
                        download_pdfs,
                        download_imgs,
                        scan_subfolders,
                    )
                    .await;
                } // for img
            } // for href
        } // for row
    } // for table
      //
}

pub async fn get_images(
    img: ElementRef<'static>,
    href: ElementRef<'static>,
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("{} GETTING IMAGEES", "[Crawler]".bold().red());

    let href_attr = href.value().attr("href").unwrap();
    // get all images from row

    let is_directory = "[DIR]";
    let is_image = "[IMG]";
    let is_pdf = "[   ]";
    let is_parent_directory = "[PARENTDIR]";
    let is_icon = "[ICO]";

    if let Some(alt) = img.value().attr("alt") {
        if alt == is_parent_directory || alt == is_icon {
        } else {
            let href_link = url.to_string() + href_attr; // Link obtained by looking inside the url
            let file_name = href_attr.split('/').last().unwrap_or("unknown");
            let folder: String = file_path.to_string()
                + (href_attr
                    .split('/')
                    .take(2)
                    .collect::<Vec<&str>>()
                    .join("/"))
                .as_str();
            let folder_to_download = folder.replace(file_name, "");

            println!(
                "{} Link: {}",
                "[Crawler]".bold().red(),
                href_link.bright_green().bold()
            );

            let href_attr = href.value().attr("href").unwrap();
            // else was here
            if alt == is_directory {
                if scan_subfolders == 'y' || scan_subfolders == 'Y' {
                    unsafe {
                        println!("{} Creating DIR", "[Crawler]".bold().red());
                        fs::create_dir_all(CURRENT_DIRECTORY.to_string() + href_attr)
                            .unwrap_or_else(|why| {
                                println!("{} ! {:?}", "[Crawler]".bold().red(), why);
                            });

                        // Bcz it can get infinitelty long so we use box::pin
                        Box::pin(get_table(
                            (url.to_string() + href_attr).as_str(),
                            folder.as_str(),
                            download_pdfs,
                            download_imgs,
                            scan_subfolders,
                        ))
                        .await?; // Call get_table function with new url
                    }
                }
            } else if alt == is_image {
                if download_imgs == 'y' || download_imgs == 'Y' {
                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download)
                        .await?;
                } else {
                    println!("{} Found img but, didnt download", "[Crawler]".bold().red());
                }
            } else if alt == is_pdf {
                if download_pdfs == 'y' || download_pdfs == 'Y' {
                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download)
                        .await?;
                } else {
                    println!("{} Found pdf but, didnt download", "[Crawler]".bold().red());
                }
            } else {
                println!(
                    "{} {}{}",
                    "[Crawler]".bold().red(),
                    url.bright_yellow(),
                    href_attr.bright_yellow()
                );
            }
        }
    }
    Ok(None)
}

pub fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    // Open file if u can only, other wise do floss dance WHAAATTTTTTTTTTTT :{}
    let file = File::open(path)?;
    // Read the files and convert it into Buffer
    let reader = BufReader::new(file);
    // If OK then
    Ok(
        // Return the lines of the file
        reader.lines().filter_map(Result::ok).collect(),
    )
}

pub fn create_directory_if_it_does_not_exist(directory_path: &str) {
    if !fs::metadata(directory_path).is_ok() {
        fs::create_dir_all(directory_path).unwrap_or_else(|why| {
            println!("{} ! {:?}", "[Crawler]".bold().red(), why);
        });
    }
}

pub async fn download_file_from_url_with_folder(
    url: &str,
    input_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    create_directory_if_it_does_not_exist(input_path);

    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    // https://dl.chughtailibrary.com/files/repository/book_quest/history_geography/2/pdf_images/
    //
    let file_name = url.split('/').last().unwrap_or("unknown");

    let file_type = file_name.split('.').last().unwrap_or("unknown");

    let path = input_path.to_string() + file_name;

    let mb = bytes.len() / (1024 * 1024);

    println!(
        "{} {} {} | {} {} | {} {} MB | Path {}",
        "[Crawler]".bold().red(),
        //  Headings in bold     variables with colors
        "File Type:".red().underline(),
        file_type.bold().bright_purple(),
        "File Name:".green().underline(),
        file_name.bold().bright_yellow(),
        "File Size:".blue().underline(),
        mb.to_string().bold().bright_cyan(),
        path.magenta()
    );

    println!(
        "{} {} | {}",
        "[Crawler]".bold().red(),
        "Downloading at".underline().bold(),
        path
    );

    let file_path = Path::new(&path); // added &
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;

    Ok(())
}

pub async fn read_urls_from_file(
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{} {}",
        "[Crawler]".bold().red(),
        "******* Reading URLS *******".bold().underline().green()
    );
    // Get all the urls from the file :D and save it into a vector of type string
    let paths: Vec<String> = read_lines("urls.txt")?; // ? does the thing only if there is no error

    for path in paths {
        let _ = get_table(
            path.as_str(),
            "Download/",
            download_pdfs,
            download_imgs,
            scan_subfolders,
        )
        .await;
    }
    println!("{} READ ALL URLS", "[Crawler]".bold().red());
    return Ok(());
}

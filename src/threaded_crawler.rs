// <-----------> Importing standard libraries <-----------
pub static mut LAST_FILE_PATH: Option<String> = None;

#[allow(static_mut_refs)]
pub static mut DOWNLOADABLE_FILES: Vec<DownloadableFile> = vec![];
// std(static)
//use std::env;
use std::fs;
use std::fs::File;
//use std::io;
use std::io::prelude::*;
// use std::io::BufReader;
use std::path::Path;
//use std::thread;
// use(s)
use colored::Colorize;
use futures::stream::{FuturesUnordered, StreamExt};

pub struct DownloadableFile {
    url: String,
    input_path: String,
}

//use gtk::prelude::*;
//use gtk::{
//    glib, Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, Orientation,
//};
//use gtk4 as gtk;
//use gtk4::cairo::ffi::STATUS_SUCCESS;

use reqwest::Client;
use scraper::ElementRef;
use scraper::{Html, Selector};

use crate::DISPLAY_DEBUG_INFO;
use crate::OVERRIDE_EXISTING_FILES;

macro_rules! logln {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            "[ThreadedCrawler]".bold().truecolor(255, 192, 203),
            format!($($arg)*)
        );
    };
}

pub async fn download_threaded(
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut tasks = FuturesUnordered::new();
    logln!("Getting table url: {}", url);
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
    logln!("Starting Download...");
    unsafe {
        //DOWNLOADABLE_FILES.par_iter().for_each(|downloadable_file| {
        //    download_file_from_url_with_folder(downloadable_file).await;
        //});
        #[allow(static_mut_refs)]
        while let Some(file) = DOWNLOADABLE_FILES.pop() {
            tasks.push(async move {
                if let Err(e) = download_file_from_url_with_folder(&file).await {
                    eprintln!("Failed to download: {}", e);
                }
            });
        }
    }
    // Await all download tasks
    while tasks.next().await.is_some() {}
    logln!("Downloads finished...");
    Ok(None) // return None
}

async fn get_table(
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    logln!("Getting table url: {}", url);
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

async fn extract_table(
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
    html: &'static Html,
) {
    unsafe {
        if DISPLAY_DEBUG_INFO {
            logln!("EXTRACTING TABLE.....");
        }
    }

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

async fn get_images(
    img: ElementRef<'static>,
    href: ElementRef<'static>,
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    // if url doenst end with / add a /
    // causes issues if not done like this
    let mut url = url.to_string();

    if !url.ends_with('/') {
        url.push('/');
    }

    let url = url.as_str();
    unsafe {
        if DISPLAY_DEBUG_INFO {
            logln!("Getting Images and PDFs....");
        }
    };

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
            // https://dl.chughtailibrary.com/files/repository/book_quest/history_geography/2/pdf_images/
            let href_link = url.to_string() + href_attr; // Link obtained by looking inside the url
            let file_name = href_attr.split('/').last().unwrap_or("unknown");
            let folder: String =
                file_path.to_string() + (url.strip_prefix("https://").unwrap_or(url));
            let folder_to_download = folder.replace(file_name, "");
            // logln!("url:{} href_attr:{} href_link:{} file_name:{} folder:{} path:{}", url.bright_green(), href_attr.bright_green(), href_link.bright_green(), file_name.bright_green(), folder.bright_green(), file_path.bright_green());
            unsafe {
                if DISPLAY_DEBUG_INFO {
                    println!(
                        "{} Link: {}",
                        "[Crawler]".bold().red(),
                        href_link.bright_green().bold()
                    )
                }
            };

            let href_attr = href.value().attr("href").unwrap();
            // else was here
            if alt == is_directory {
                if scan_subfolders == 'y' || scan_subfolders == 'Y' {
                    unsafe {
                        if DISPLAY_DEBUG_INFO {
                            logln!("GOING TO URL: {}", (url.to_string() + href_attr).as_str());
                        }
                    }
                    // OLD CODE NEEDED IT, NOW THE NEW ONE DOESNT
                    // println!("{} Creating DIR", "[Crawler]".bold().red());

                    //fs::create_dir_all(CURRENT_DIRECTORY.to_string() + href_attr)
                    //    .unwrap_or_else(|why| {
                    //        println!("{} ! {:?}", "[Crawler]".bold().red(), why);
                    //    });

                    // Bcz it can get infinitelty long so we use box::pin
                    Box::pin(get_table(
                        (url.to_string() + href_attr).as_str(),
                        "Download/",
                        download_pdfs,
                        download_imgs,
                        scan_subfolders,
                    ))
                    .await?; // Call get_table function with new url
                }
            } else if alt == is_image {
                if download_imgs == 'y' || download_imgs == 'Y' {
                    let downloadable_file = DownloadableFile {
                        url: href_link,
                        input_path: folder_to_download,
                    };
                    unsafe {
                        #[allow(static_mut_refs)]
                        DOWNLOADABLE_FILES.push(downloadable_file);
                    }
                    //download_file_from_url_with_folder(downloadable_file).await?;
                } else {
                    unsafe {
                        if DISPLAY_DEBUG_INFO {
                            logln!("Found Image, didnt download {}", href_link);
                        }
                    };
                }
            } else if alt == is_pdf {
                if download_pdfs == 'y' || download_pdfs == 'Y' {
                    let downloadable_file = DownloadableFile {
                        url: href_link,
                        input_path: folder_to_download,
                    };
                    unsafe {
                        #[allow(static_mut_refs)]
                        DOWNLOADABLE_FILES.push(downloadable_file);
                    }
                    //download_file_from_url_with_folder(downloadable_file).await?;
                } else {
                    unsafe {
                        if DISPLAY_DEBUG_INFO {
                            logln!("Found PDF, didnt download {}", href_link);
                        }
                    };
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

fn create_directory_if_it_does_not_exist(directory_path: &str) {
    if !fs::metadata(directory_path).is_ok() {
        fs::create_dir_all(directory_path).unwrap_or_else(|why| {
            logln!("! {:?}", why);
        });
    }
}

async fn download_file_from_url_with_folder(
    downloadable_file: &DownloadableFile,
) -> Result<(), Box<dyn std::error::Error>> {
    let url: &str = &downloadable_file.url;
    let input_path: &str = &downloadable_file.input_path;
    unsafe {
        create_directory_if_it_does_not_exist(input_path);
        let file_name = url.split('/').last().unwrap_or("unknown");

        let file_type = file_name.split('.').last().unwrap_or("unknown");

        let path = input_path.to_string() + file_name;
        let file_path = Path::new(&path); // added &

        LAST_FILE_PATH = Some(input_path.to_string());

        if file_path.exists() {
            if DISPLAY_DEBUG_INFO {
                logln!("{} already exists", path);
            }

            if OVERRIDE_EXISTING_FILES {
                logln!("ovverinding file: {}", path);
                // SEND REQUEST TO GET THE FILE
                download_file(file_name, file_type, &path, file_path, url).await?
            }
        } else {
            logln!("downloading file at: {}", path);
            download_file(file_name, file_type, &path, file_path, url).await?;
        }
    }
    Ok(())
}

async fn download_file(
    file_name: &str,
    file_type: &str,
    path: &String,
    file_path: &Path,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    // https://dl.chughtailibrary.com/files/repository/book_quest/history_geography/2/pdf_images/
    //

    let mb = bytes.len() / (1024 * 1024);

    unsafe {
        if DISPLAY_DEBUG_INFO {
            logln!(
                "{} {} | {} {} | {} {} MB | Path {}",
                //  Headings in bold     variables with colors
                "File Type:".red().underline(),
                file_type.bold().bright_purple(),
                "File Name:".green().underline(),
                file_name.bold().bright_yellow(),
                "File Size:".blue().underline(),
                mb.to_string().bold().bright_cyan(),
                path.magenta()
            );
        }
        logln!("{} | {}", "Downloading at".underline().bold(), path);
    }
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;

    Ok(())
}

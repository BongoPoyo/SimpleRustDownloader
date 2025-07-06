// <-----------> Importing standard libraries <----------->
static mut CURRENT_DIRECTORY: &str = "Download/";
// std(s)
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::thread;
// use(s)
use colored::Colorize;
use colored::*;
use gtk::prelude::*;
use gtk::{
    glib, Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, Orientation,
};
use gtk4 as gtk;
use gtk4::cairo::ffi::STATUS_SUCCESS;
use jpeg_to_pdf::JpegToPdf;
use reqwest::Client;
use scraper::ElementRef;
use scraper::{Html, Selector};

async fn get_table(
    url: &str,
    file_path: &str,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("GETTING TABLE");
    println!("URL : {}", url.bold().blink()); // Print the URL

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
    println!("EXTRACTING TABLE");
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

                let href_attr = href.value().attr("href").unwrap(); // gets the href attribute

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
    println!("GETTING IMAGEES");
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

            println!("Link: {}", href_link.bright_green().bold());

            let href_attr = href.value().attr("href").unwrap();
            // else was here
            if alt == is_directory {
                if scan_subfolders == 'y' || scan_subfolders == 'Y' {
                    unsafe {
                        println!("Creating DIR");
                        fs::create_dir_all(CURRENT_DIRECTORY.to_string() + href_attr)
                            .unwrap_or_else(|why| {
                                println!("! {:?}", why);
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
                    println!("Found img but, didnt download");
                }
            } else if alt == is_pdf {
                if download_pdfs == 'y' || download_pdfs == 'Y' {
                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download)
                        .await?;
                } else {
                    println!("Found pdf but, didnt download");
                }
            } else {
                println!("{}{}", url.bright_yellow(), href_attr.bright_yellow());
            }
        }
    }
    Ok(None)
}

fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
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

fn create_directory_if_it_does_not_exist(directory_path: &str) {
    if !fs::metadata(directory_path).is_ok() {
        fs::create_dir_all(directory_path).unwrap_or_else(|why| {
            println!("! {:?}", why);
        });
    }
}
async fn download_file_from_url_with_folder(
    url: &str,
    input_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    create_directory_if_it_does_not_exist(input_path);

    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    let file_name = url.split('/').last().unwrap_or("unknown");

    let file_type = file_name.split('.').last().unwrap_or("unknown");

    let path = input_path.to_string() + file_name;

    let mb = bytes.len() / (1024 * 1024);

    println!(
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

    println!("{} | {}", "Downloading at".underline().bold(), path);

    let file_path = Path::new(&path); // added &
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;

    Ok(())
}

async fn read_urls(
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
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
    println!("READ ALL URLS");
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        "******* PASS IN `--cli` to use cli *******"
            .bold()
            .bright_purple()
            .underline()
    );

    let mut args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("NO COMMANDS PASSED IN");
        args.push("gui".to_string());
        args.push("".to_string());
    }
    let query = &args[1];
    match query.as_str() {
        "--cli" => {
            println!("RUNNING CLI")
        }
        _ => {
            println!("RUNNING GUI");
            let result = create_application().await;
            match result {
                glib::ExitCode::SUCCESS => {
                    panic!("_ UI CLOSED");
                }
                _ => {}
            }
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
            println!("URL not passed in");
            let _ = read_urls(download_pdfs, download_imgs, scan_subfolders);
        }
        _ => {
            println!("URL: {}", url.red());
            let _ = get_table(
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
        "******* Task Completed! Press Enter to exit *******"
            .bold()
            .underline()
            .red()
    );
    let mut choice: String = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to readline");
    Ok(()) // Return statement
}

async fn create_application() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.FSS")
        .build();

    app.connect_activate(|app| {
        // Create main window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Book Downloader")
            .default_width(980)
            .default_height(420)
            .build();

        // Create vertical layout
        let vbox = GtkBox::new(Orientation::Vertical, 10);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);

        // Create checkboxes
        let pdf_checkbox = CheckButton::with_label("PDF");
        let image_checkbox = CheckButton::with_label("Image");
        let subfolder_checkbox = CheckButton::with_label("Subfolder");
        
        let url_label = gtk4::Label::new(Some("URL:"));
        
        let url_entry = gtk4::Entry::new();
        url_entry.set_placeholder_text(Some("Enter a URL..."));
        url_entry.set_text("https://dl.chughtailibrary.com/files/repository/book_quest/history_geography/2/pdf_images/");
        url_entry.set_hexpand(true);
        // Create Download button
        let download_button = Button::with_label("Download");

        // Handle button click event
        let pdf_cb = pdf_checkbox.clone();
        let image_cb = image_checkbox.clone();
        let subfolder_cb = subfolder_checkbox.clone();
        let url_entry_cb = url_entry.clone();

        download_button.connect_clicked(move |_| {
            let pdf_cb = pdf_cb.clone();
            let image_cb = image_cb.clone();
            let subfolder_cb = subfolder_cb.clone();
            let url_entry_cb = url_entry_cb.clone();

            println!(
                "Download options: PDF = {}, Image = {}, Subfolder = {}, URL = {}",
                pdf_cb.is_active(),
                image_cb.is_active(),
                subfolder_cb.is_active(),
                url_entry_cb.text().to_string()
            );

            let mut download_pdfs = "n".trim().chars().next().unwrap();
            let mut download_imgs = "n".trim().chars().next().unwrap();
            let mut scan_subfolders = "n".trim().chars().next().unwrap();

            // poorest code i have ever written
            glib::MainContext::default().spawn_local(async move {
                if pdf_cb.is_active() {
                    download_pdfs = "y".trim().chars().next().unwrap();
                }
                if image_cb.is_active() {
                    download_imgs = "y".trim().chars().next().unwrap();
                }
                if subfolder_cb.is_active() {
                    scan_subfolders = "y".trim().chars().next().unwrap();
                }
                let _ = get_table(
                    &url_entry_cb.text(),
                    "Download/",
                    download_pdfs,
                    download_imgs,
                    scan_subfolders,
                ).await;
            });
        });

        // Add widgets to layout
        //
        let hbox_checkboxes = GtkBox::new(Orientation::Horizontal, 10);
        hbox_checkboxes.set_margin_top(20);
        hbox_checkboxes.set_margin_bottom(20);
        hbox_checkboxes.set_margin_start(20);
        hbox_checkboxes.set_margin_end(20);

        hbox_checkboxes.append(&pdf_checkbox);
        hbox_checkboxes.append(&image_checkbox);
        hbox_checkboxes.append(&subfolder_checkbox);
        hbox_checkboxes.set_halign(Align::Center);
        
        let hbox_url = GtkBox::new(Orientation::Horizontal, 10);
        
        hbox_url.set_margin_top(20);
        hbox_url.set_margin_bottom(20);
        hbox_url.set_margin_start(20);
        hbox_url.set_margin_end(20);
        
        hbox_url.append(&url_label);
        hbox_url.append(&url_entry);

        vbox.append(&hbox_url);
        vbox.append(&hbox_checkboxes);
        vbox.append(&download_button);
        // Center align button
        download_button.set_halign(Align::Center);
        // Add layout to window
        window.set_child(Some(&vbox));
        window.show();
    });

    app.run()
}

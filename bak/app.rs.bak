//use std::env;
//use std::fs;
//use std::fs::File;
//use std::io;
//use std::io::prelude::*;
//use std::io::BufReader;
//use std::path::Path;
//use std::thread;
// use(s)
//use colored::Colorize;
//use colored::*;
use crate::crawler;
use gtk::prelude::*;
use gtk::{
    glib, Align, Application, ApplicationWindow, Box as GtkBox, Button, CheckButton, Orientation,
};
use gtk4 as gtk;
//use gtk4::cairo::ffi::STATUS_SUCCESS;
//use jpeg_to_pdf::JpegToPdf;
//use reqwest::Client;
//use scraper::ElementRef;
//use scraper::{Html, Selector};



pub async fn create_application() -> glib::ExitCode {
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
            let download_pdfs = if pdf_cb.is_active() { 'y' } else { 'n' };
            let download_imgs = if image_cb.is_active() { 'y' } else { 'n' };
            let scan_subfolders = if subfolder_cb.is_active() { 'y' } else { 'n' };
            let url : String = url_entry_cb.text().to_string();



            println!(
                "Download options: PDF = {}, Image = {}, Subfolder = {}, URL = {}",
                download_pdfs, download_imgs, scan_subfolders, url
            );


            // poorest code i have ever written
            //std::thread::spawn(
                
            glib::MainContext::default().spawn_local(
                async move {

                match crawler::get_table(
                    &url.as_str(),
                    "Download/",
                    download_pdfs,
                    download_imgs,
                    scan_subfolders,
                ).await {
                    Ok(_) => print!("Download Completed"),
                    Err(e) => println!("Get Table err: {:?}", e),
                }
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



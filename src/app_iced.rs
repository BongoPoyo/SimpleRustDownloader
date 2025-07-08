#![allow(unused_imports)]
use crate::crawler;
use crate::pdf_maker;
use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::{executor, Alignment, Element, Settings};
use std::thread;
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
struct State {
    url: String,
    download_pdfs: bool,
    download_imgs: bool,
    scan_subfolders: bool,
    is_downloading: bool,
    //downloaded: bool,
}

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    PdfToggle(bool),
    ImgToggle(bool),
    SubfolderToggle(bool),
    DownloadPressed,
    ConvertToPdfPressed,
}

impl Default for State {
    fn default() -> Self {
        Self {
            url: "https://dl.chughtailibrary.com/files/repository/book_quest/history_geography/2/pdf_images/".to_string(), // <- your default URL
            download_pdfs: true,
            download_imgs: true,
            scan_subfolders: true,
            is_downloading: false,
        }
    }
}

pub fn create_application() -> iced::Result {
    iced::run("Downloader App", update, view)
}

fn update(state: &mut State, message: Message) {
    if !state.is_downloading {
        match message {
            Message::UrlChanged(new_url) => state.url = new_url,
            Message::PdfToggle(v) => state.download_pdfs = v,
            Message::ImgToggle(v) => state.download_imgs = v,
            Message::SubfolderToggle(v) => state.scan_subfolders = v,
            Message::DownloadPressed => {
                let url = state.url.clone();
                let download_pdfs = if state.download_pdfs { 'y' } else { 'n' };
                let download_imgs = if state.download_imgs { 'y' } else { 'n' };
                let scan_subfolders = if state.scan_subfolders { 'y' } else { 'n' };
                state.is_downloading = true;
                thread::spawn(move || {
                    // Create a new Tokio runtime for this thread
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async move {
                        let result = crawler::get_table(
                            &url.as_str(),
                            "Download/",
                            download_pdfs,
                            download_imgs,
                            scan_subfolders,
                        )
                        .await;

                        match result {
                            Ok(_) => {
                                println!("Download Completed");
                                //state.downloaded = true;
                            }
                            Err(e) => {
                                println!("Exited with error {}", e);
                            }
                        }
                    });
                })
                .join()
                .expect("Error joining thread");
                println!("Thread joined :>");
                state.is_downloading = false
            }
            Message::ConvertToPdfPressed => {
                thread::spawn(move || {
                    println!("CONVERT BTN PRESSED...");
                    let _ = pdf_maker::convert_jpegs_to_pdf();
                })
                .join()
                .expect("ERROR JOINING PDF THREAD");

                println!("PDF COMPRESSED");
            }
        }
    } else {
        println!("Wait is downloading");
    }
}

fn view(state: &State) -> Element<Message> {
    let mut download_button = button("Download").on_press(Message::DownloadPressed);
    let mut convert_pdf_button =
        button("Convert Img to PDF").on_press(Message::ConvertToPdfPressed);

    if state.is_downloading {
        download_button = button("Downloading");
        convert_pdf_button = button("Wait for downloading to complete...");
    }

    column![
        text("Enter URL:"),
        text_input("https://example.com", &state.url)
            .on_input(Message::UrlChanged)
            .padding(10),
        row![
            checkbox("Download PDF", state.download_pdfs).on_toggle(Message::PdfToggle),
            checkbox("Download Images", state.download_imgs).on_toggle(Message::ImgToggle),
            checkbox("Scan Subfolders", state.scan_subfolders).on_toggle(Message::SubfolderToggle)
        ]
        .spacing(20),
        row![download_button, convert_pdf_button,].spacing(30)
    ]
    .spacing(15)
    .padding(20)
    .align_x(Alignment::Start)
    .into()
}

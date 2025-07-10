#![allow(unused_imports)]

use crate::crawler;
use crate::pdf_maker;
use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::Task;
use iced::Theme;
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
    DownloadFinished(String),
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
    iced::application("Downloader App", update, view)
        .theme(theme)
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::UrlChanged(new_url) => {
            state.url = new_url;
            Task::none()
        }
        Message::PdfToggle(v) => {
            state.download_pdfs = v;
            Task::none()
        }
        Message::ImgToggle(v) => {
            state.download_imgs = v;
            Task::none()
        }
        Message::SubfolderToggle(v) => {
            state.scan_subfolders = v;
            Task::none()
        }
        Message::DownloadPressed => {
            state.is_downloading = true;
            let url = state.url.clone();
            let download_pdfs = if state.download_pdfs { 'y' } else { 'n' };
            let download_imgs = if state.download_imgs { 'y' } else { 'n' };
            let scan_subfolders = if state.scan_subfolders { 'y' } else { 'n' };

            Task::perform(
                download(url, download_pdfs, download_imgs, scan_subfolders),
                Message::DownloadFinished,
            )
        }
        Message::ConvertToPdfPressed => {
            thread::spawn(move || {
                println!("[APP-ICED] CONVERT BTN PRESSED...");
                let _ = pdf_maker::convert_jpegs_to_pdf();
            })
            .join()
            .unwrap();

            println!("[APP-ICED] PDF COMPRESSED");
            Task::none()
        }
        Message::DownloadFinished(_string) => {
            state.is_downloading = false;

            Task::none()
        }
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

fn theme(_state: &State) -> Theme {
    Theme::TokyoNight
}

async fn download(
    url: String,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> String {
    thread::spawn(move || {
        // Create a new Tokio runtime for this thread
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let _ = crawler::get_table(
                &url.as_str(),
                "Download/",
                download_pdfs,
                download_imgs,
                scan_subfolders,
            )
            .await;
        });

        println!("[APP-ICED] THE END");
    })
    .join()
    .unwrap();

    String::from("Hehe")
}

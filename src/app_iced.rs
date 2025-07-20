//#![allow(unused_imports)]
//#![allow(dead_code)]
//#![allow(static_mut_refs)]
//static DOWNLOAD_THREAD_HANDLE: OnceCell<JoinHandle<()>> = OnceCell::new();

//static DOWNLOAD_THREAD: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

use crate::pdf_maker;
use crate::threaded_crawler;
use crate::threaded_crawler::STOP_SCANNING_FILES;
use crate::OVERRIDE_EXISTING_FILES;
use colored::Colorize;
use dark_light::Mode;
use iced::widget::{button, checkbox, column, row, text, text_input};
use iced::Task;
use iced::Theme;
use iced::{Alignment, Element};
use notify_rust::Notification;
use opener;
use std::path::Path;
use std::thread::{self};
use tokio::runtime::Runtime;

macro_rules! logln {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            "[AppIced]".bold().blue(),
            format!($($arg)*)
        );
    };
}
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
    OverrideToggle(bool),
    OpenFileExplorerPressed,
    StopScanning,
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
            unsafe {
                threaded_crawler::STOP_SCANNING_FILES = false;
            }
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
                println!("{} CONVERT BTN PRESSED...", "[AppIced]".blue().bold());
                let _ = pdf_maker::convert_jpegs_to_pdf();
            })
            .join()
            .unwrap();
            match Notification::new()
                .summary("Simple Rust Downloader")
                .body("PDF Compressed")
                .show()
            {
                Ok(_) => {}
                Err(e) => {
                    logln!("Error sending notification: {}", e);
                }
            }
            println!("{} PDF COMPRESSED", "[AppIced]".blue().bold());
            Task::none()
        }
        Message::DownloadFinished(_string) => {
            state.is_downloading = false;

            unsafe {
                #[allow(static_mut_refs)]
                if let Some(value) = &threaded_crawler::LAST_FILE_PATH {
                    logln!("Last file was {}", value);
                }
            }

            match Notification::new()
                .summary("Simple Rust Downloader")
                .body("Downloadig finished")
                .action("convert", "Convert to PDF")
                .show()
            {
                Ok(_) => {}
                Err(e) => {
                    logln!("error showing notification: {}", e);
                }
            }
            Task::none()
        }
        Message::OverrideToggle(bool) => {
            unsafe {
                OVERRIDE_EXISTING_FILES = bool;
            }

            Task::none()
        }
        Message::OpenFileExplorerPressed => {
            logln!("Opening file");

            unsafe {
                #[allow(static_mut_refs)]
                if let Some(s) = &threaded_crawler::LAST_FILE_PATH {
                    let path = Path::new(s.as_str());
                    opener::open(path).expect("Error opening path");
                }
            }
            Task::none()
        }
        Message::StopScanning => {
            logln!("Stopping...");
            unsafe {
                threaded_crawler::STOP_SCANNING_FILES = true;
            }
            Task::none()
        }
    }
}

fn view(state: &State) -> Element<Message> {
    let mut download_button = button("Download").on_press(Message::DownloadPressed);
    let open_file_explorer_button =
        button("📂 Open Downloads Folder").on_press(Message::OpenFileExplorerPressed);
    // let mut cancel_button = button("Cancel Download").on_press(Message::CancelDownload);
    let mut convert_pdf_button =
        button("Convert Img to PDF").on_press(Message::ConvertToPdfPressed);

    if state.is_downloading {
        unsafe {
            if STOP_SCANNING_FILES {
                download_button = button("Stopped Scanning, Waiting for downloading to finish...");
            } else {
                download_button = button("Stop Scanning?").on_press(Message::StopScanning);
            }
        }

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
            checkbox("Scan Subfolders", state.scan_subfolders).on_toggle(Message::SubfolderToggle),
            unsafe {
                checkbox("Override existing files", OVERRIDE_EXISTING_FILES)
                    .on_toggle(Message::OverrideToggle)
            }
        ]
        .spacing(20),
        row![
            download_button,
            convert_pdf_button,
            open_file_explorer_button,
        ]
        .spacing(30)
    ]
    .spacing(15)
    .padding(20)
    .align_x(Alignment::Start)
    .into()
}

fn theme(_state: &State) -> Theme {
    match dark_light::detect() {
        Ok(Mode::Dark) => Theme::CatppuccinMocha,
        Ok(Mode::Light) => Theme::Light,
        _ => Theme::CatppuccinMocha,
    }
}

async fn download(
    url: String,
    download_pdfs: char,
    download_imgs: char,
    scan_subfolders: char,
) -> String {
    // WORKING CODE BUT DOESNT SUPPORT CANCELLATION
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let _ = threaded_crawler::download_threaded(
                &url.as_str(),
                "Download/",
                download_pdfs,
                download_imgs,
                scan_subfolders,
            )
            .await;
        });

        println!("{} End Of download thread", "[AppIced]".blue().bold());
    })
    .join()
    .unwrap();

    String::from("Hehe")
}

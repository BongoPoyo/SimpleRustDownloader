use crate::crawler;
use colored::Colorize;
use jpeg_to_pdf::JpegToPdf;
use std::fs::{self, DirEntry, File};
use std::io;
use std::path::Path; //, PathBuf};
macro_rules! logln {
    ($($arg:tt)*) => {
        println!(
            "{} {}",
            "[PdfMaker]".bold().purple(),
            format!($($arg)*)
        );
    };
}

pub fn convert_jpegs_to_pdf() -> io::Result<()> {
    scan_folder(Path::new("Download/"))
}

fn scan_folder(path: &Path) -> io::Result<()> {
    let mut jpeg_to_pdf = JpegToPdf::new();

    for entry in fs::read_dir(path).expect("[PdfMaker] Cant read directory Download/") {
        let entry: DirEntry = entry.expect("[PdfMaker] DirEntry doesnt exist");
        let path = entry.path();

        if path.is_dir() {
            let _ = scan_folder(&path);
        }
        let file_type = entry
            .file_type()
            .expect("[PdfMaker] Error getting filetype");

        if file_type.is_file() {
            let file_ext = path
                .extension()
                .and_then(|ext| ext.to_str())
                .expect("[PdfMaker] Error getting file extension");

            if file_ext == "jpg" {
                jpeg_to_pdf =
                    jpeg_to_pdf.add_image(fs::read(path).expect("[PdfMaker] error reading jpg"));
            }
        }
    }

    if path
        .to_str()
        .expect("[PdfMaker] error converting to string")
        .contains("pdf_images")
    {
        let out_file_path = path
            .parent()
            .expect("[PdfMaker] ERROR GETTING FILE PARENT")
            .join("out.pdf");

        unsafe {
            let string_path = path
                .parent()
                .expect("Error getting parent folder")
                .to_string_lossy()
                .into_owned();
            logln!("Last file path is {}", string_path);
            crawler::LAST_FILE_PATH = Some(string_path);
        }

        let out_file = File::create(&out_file_path).expect("[PdfMaker] error creating file");
        logln!(
            "Converting images inside {:?} to pdf: {:?}",
            path,
            out_file_path
        );
        jpeg_to_pdf
            .create_pdf(&mut std::io::BufWriter::new(out_file))
            .expect("[PdfMaker] Error creating pdf");
    }

    Ok(())
}

use jpeg_to_pdf::JpegToPdf;
use std::fs::{self, DirEntry, File};
use std::io;
use std::path::Path; //, PathBuf};

pub fn convert_jpegs_to_pdf() -> io::Result<()> {
    scan_folder(Path::new("Download/"))
}

fn scan_folder(path: &Path) -> io::Result<()> {
    let mut jpeg_to_pdf = JpegToPdf::new();

    for entry in fs::read_dir(path).expect("Cant read directory Download/") {
        let entry: DirEntry = entry.expect("DirEntry doesnt exist");
        let path = entry.path();

        if path.is_dir() {
            let _ = scan_folder(&path);
        }
        let file_type = entry.file_type().expect("Error getting filetype");

        if file_type.is_file() {
            let file_ext = path
                .extension()
                .and_then(|ext| ext.to_str())
                .expect("Error getting file extension");

            if file_ext == "jpg" {
                jpeg_to_pdf = jpeg_to_pdf.add_image(fs::read(path).expect("error reading jpg"));
            }
        }
    }

    if path
        .to_str()
        .expect("error converting to string")
        .contains("pdf_images")
    {
        let out_file_path = path
            .parent()
            .expect("ERROR GETTING FILE PARENT")
            .join("out.pdf");

        let out_file = File::create(&out_file_path).expect("error creating file");
        println!(
            "Converting images inside {:?} to pdf: {:?}",
            path, out_file_path
        );
        jpeg_to_pdf
            .create_pdf(&mut std::io::BufWriter::new(out_file))
            .expect("Error creating pdf");
    }

    Ok(())
}

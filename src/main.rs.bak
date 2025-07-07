// <-----------> Importing standard libraries <----------->
static mut CURRENT_DIRECTORY : &str = "Download/";
// std(s)
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::io;
use std::io::BufReader;
use std::path::Path;

// use(s)
use reqwest::Client;
use colored::Colorize;
use scraper::{Html, Selector};
use colored::*;

// fn get_alt_tag(url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
//     // send a response to the url
//     let response = ureq::get(url).call()?;
//     let html = Html::parse_document(&response.into_string()?);
//     let img_selector = Selector::parse("img").unwrap();
//     // get alt tags
//     for img in html.select(&img_selector) {
//         if let Some(alt) = img.value().attr("alt") {
//             return Ok(Some(alt.to_string()));
//         }
//     }
//     Ok(None)
// }


// made it async
async fn get_table(url: &str, file_path: &str, download_pdfs:char, download_imgs:char, scan_subfolders:char) -> Result<Option<String>, Box<dyn std::error::Error>> {
    println!("URL : {}", url.bold().blink()); // Print the URL

    let response = ureq::get(url).call()?; // send a request to the url
    let html = Html::parse_document(&response.into_string()?); // parse the html from the response
    let table_selector = Selector::parse("table").unwrap(); // make table selector
    
    // Get all tables from html
    for table in html.select(&table_selector){ // get all tables from html

        let row_selector = Selector::parse("tr").unwrap(); // make a row selector
        
        // Get all rows from table
        for row in table.select(&row_selector) { // get all rows from table

            let href_selector = Selector::parse("a[href]").unwrap(); // make a href selector

            for href in row.select(&href_selector) { // Get all links from row

                let href_attr = href.value().attr("href").unwrap(); // gets the href attribute
                
                let img_selector = Selector::parse("img").unwrap();

                for img in row.select(&img_selector) {// get all images from row

                    let is_directory = "[DIR]";
                    let is_image = "[IMG]";
                    let is_pdf = "[   ]";
                    let is_parent_directory = "[PARENTDIR]";
                    let is_icon = "[ICO]";

                    if let Some(alt) = img.value().attr("alt") {
                        if alt == is_parent_directory || alt == is_icon{
                        }
                        else{

                            let href_link = url.to_string() + href_attr; // Link obtained by looking inside the url
                            let file_name = href_attr.split('/').last().unwrap_or("unknown");
                            let folder = file_path.to_string() +
                                (href_attr.split('/').take(2).collect::<Vec<&str>>().join("/")).as_str();
                            let folder_to_download = folder.replace(file_name, "");

                            println!("Link: {}", href_link.bright_green().bold());

                            let href_attr = href.value().attr("href").unwrap();
                            // if href_attr == "?C=N;O=D" || href_attr == "?C=M;O=A" || href_attr == "?C=S;O=A" || href_attr == "?C=D;O=A" {
                            //     println!("{}","Extra btns".bright_cyan());
                            // }
                            // else was here
                            if alt == is_directory {
                                if scan_subfolders == 'y' || scan_subfolders == 'Y'{
                                    unsafe
                                    {
                                        fs::create_dir_all(CURRENT_DIRECTORY.to_string() + href_attr).unwrap_or_else(|why| {
                                            println!("! {:?}", why);
                                        });
                                        // STILL Needs
                                        // CURRENT_DIRECTORY = (CURRENT_DIRECTORY.to_string() + href_attr).as_str();
    
                                        // Bcz it can get infinitelty long so we use box::pin
                                        Box::pin(get_table((url.to_string() + href_attr).as_str(), folder.as_str(),
                                         download_pdfs, download_imgs, scan_subfolders)).await?; // Call get_table function with new url
                                    }
                                }
                               
                            }

                            else if alt == is_image{
                                if download_imgs == 'y' || download_imgs == 'Y'{
                                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download).await?;
                                }
                                else{
                                    println!("Found img but, didnt download");
                                }
                        
                            }
                            else if alt == is_pdf{
                                if download_pdfs == 'y' || download_pdfs == 'Y' {
                                    download_file_from_url_with_folder(&href_link.as_str(), &folder_to_download).await?;
                                }
                                else{
                                    println!("Found pdf but, didnt download");
                                }
                                
                            }
                            else{
                                println!("{}{}",url.bright_yellow(), href_attr.bright_yellow());
                            }
                        }
                        


                    }// if let Some(alt) = img.value().attr("alt")
                }// for img

                // <--------------> Code for http://www.tajalli.in/pdfs.asp <-------------------->
                
                // let file_name = href_attr.split('/').last().unwrap_or("unknown");
                // let file_type = href_attr.split('.').last().unwrap_or("unknown");
                // let folder = "Download/".to_string() +
                //     (href_attr.split('/').take(2).collect::<Vec<&str>>().join("/")).as_str() + "/";
                // if file_type == "asp"{

                // }
                // else {

                //     let parent_url = url.replace("pdfs.asp", "");

                //     let folder_to_create = folder.as_str();
                //     create_directory_if_it_does_not_exist(&folder_to_create);
                    
                //     let file_to_download = parent_url + href_attr;

                //     println!("URL inide href: {} | File Name: {} | File Type: {} | Folder {}, Link {}",
                //         href_attr.bright_green(),
                //         file_name.purple(),
                //         file_type.bright_yellow(),
                //         folder.red(),
                //         file_to_download.cyan()
                //     );

                //     // if we fix this function the whole code will work for tajalli only
                //     download_file_from_url_with_folder(file_to_download.as_str(), &folder).await?;
                // }
                

                // This code will download files other than dl.chughtailibrary.com
                // download_file(text, file_name, file_type, path);
                
            }// for href

            // println!("{}", row.inner_html().bright_green());

        }// for row

        // println!("{}" , table.inner_html().red());

    }// for table

    Ok(None) // return None
}


fn read_lines(path: &str) -> std::io::Result<Vec<String>> {
    // Open file if u can only, other wise do floss dance WHAAATTTTTTTTTTTT :{}
    let file = File::open(path)?;
    // Read the files and convert it into Buffer
    let reader = BufReader::new(file);
    // If OK then 
    Ok(
        // Return the lines of the file
        reader.lines().filter_map(Result::ok).collect()
    )
}

// async fn download_file(text : String, file_name:&str, file_type: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     // let client = Client::new();
//     // let response = client.get(url).send().await?;

//     // let path_to_download = (CURRENT_DIRECTORY.to_string() + "/" +  file_name);
//     let bytes = text.as_bytes();
//     let mb = text.len() / (1024 * 1024);

//     println!("{} {} | {} {} | {} {} MB",
//     //  Headings in bold     variables with colors
//         "File Type:".red().underline(), file_type.bold().purple(),
//         "File Name:".green().underline(), file_name.bold().purple(),
//         "File Size:".blue().underline(), mb.to_string().bold().purple()
//     );

//     let file_path = Path::new(path);
//     let mut file = File::create(file_path)?;
//     file.write_all(&bytes)?;

//     Ok(())
// }

fn create_directory_if_it_does_not_exist(directory_path: &str) {
    if !fs::metadata(directory_path).is_ok() {
        fs::create_dir_all(directory_path).unwrap_or_else(|why| {
            println!("! {:?}", why);
        });
    }
}

// async fn download_file_from_url(url : &str) -> Result<(), Box<dyn std::error::Error>> {
//     unsafe{
//         create_directory_if_it_does_not_exist(CURRENT_DIRECTORY);
//     }

//     let client = Client::new();
//     let response = client.get(url).send().await?;
//     let bytes = response.bytes().await?;

//     let file_name = url.split('/').last().unwrap_or("unknown");

//     let file_type = file_name.split('.').last().unwrap_or("unknown");

//     unsafe {
//         let path = CURRENT_DIRECTORY.to_string() + file_name;
//         // let path_to_download = (CURRENT_DIRECTORY.to_string() + "/" +  file_name);
//         let mb = bytes.len() / (1024 * 1024);

//         println!("{} {} | {} {} | {} {} MB",
//         //  Headings in bold     variables with colors
//             "File Type:".red().underline(), file_type.bold().purple(),
//             "File Name:".green().underline(), file_name.bold().purple(),
//             "File Size:".blue().underline(), mb.to_string().bold().purple()
//         );
//         println!("FOLDER:{}", CURRENT_DIRECTORY);
//         let file_path = Path::new(&path); // added &
//         let mut file = File::create(file_path)?;
//         file.write_all(&bytes)?;
//     }

//     Ok(())
// }

async fn download_file_from_url_with_folder(url : &str, input_path:&str) -> Result<(), Box<dyn std::error::Error>> {

    create_directory_if_it_does_not_exist(input_path);

    let client = Client::new();
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;

    let file_name = url.split('/').last().unwrap_or("unknown");

    let file_type = file_name.split('.').last().unwrap_or("unknown");

    let path = input_path.to_string() + file_name;
    // let path_to_download = (CURRENT_DIRECTORY.to_string() + "/" +  file_name);
    let mb = bytes.len() / (1024 * 1024);

    println!("{} {} | {} {} | {} {} MB | Path {}",
    //  Headings in bold     variables with colors
        "File Type:".red().underline(), file_type.bold().bright_purple(),
        "File Name:".green().underline(), file_name.bold().bright_yellow(),
        "File Size:".blue().underline(), mb.to_string().bold().bright_cyan(),
        path.magenta()
    );

    println!("{} | {}","Downloading at".underline().bold(), path);

    let file_path = Path::new(&path); // added &
    let mut file = File::create(file_path)?;
    file.write_all(&bytes)?;

        Ok(())
    }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    control::set_virtual_terminal(true).unwrap();

    println!("{}",
        "******* Downloader Started *******".bold().bright_cyan().underline()
    );

    // let timer_start = Instant::now();// Start the timer

    println!("Download PDFs(Enter {} for yes or {} for no):", "y".bold().green(), "n".bold().red());
    let mut download_pdfs:String = String::new();
    io::stdin().read_line(&mut download_pdfs).expect("failed to readline");
    let download_pdfs = download_pdfs.trim().chars().next().unwrap();

    println!("Download Images(Enter {} for yes or {} for no):", "y".bold().green(), "n".bold().red());
    let mut download_imgs:String = String::new();
    io::stdin().read_line(&mut download_imgs).expect("failed to readline");
    let download_imgs = download_imgs.trim().chars().next().unwrap();

    println!("Scan All Subfolders(Enter {} for yes or {} for no):", "y".bold().green(), "n".bold().red());
    let mut scan_subfolders:String = String::new();
    io::stdin().read_line(&mut scan_subfolders).expect("failed to readline");
    let scan_subfolders = scan_subfolders.trim().chars().next().unwrap();

    // Get all the urls from the file :D and save it into a vector of type string
    let paths: Vec<String> = read_lines("urls.txt")?; // ? does the thing only if there is no error
    

    for path in paths{
        let _ = get_table(path.as_str(), "Download/", download_pdfs, download_imgs, scan_subfolders).await;
    }
    
    // <-------------> Download Code <-------------------->
    // let download_url = "https://ebooksapi.rekhta.org/images/6d80bb24-5c02-4215-867d-94008b5f92b1/002.jpg";
    // let _ = download_file_from_url(download_url).await;

    // let duration = timer_start.elapsed();

    // println!("{} | {:?}",
    //     "Time Taken".bright_magenta(),duration
    // );

    println!("Closing the application, press enter to close it");
    let mut choice:String = String::new();
    io::stdin().read_line(&mut choice).expect("failed to readline");
    Ok(()) // Return statement
}

// hashir ne yeh jo code copy kiya hai pta nahi kya kiya hai or usse abhi syntax bhi 
//nahi aata or wo code copy kar ke keh raha hai ke ab program ban jaye ga use kon samjaye ga 

//<-------------> Purana code hai :)
  
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("{}",
//         "******* MOON RUNE DEMOLISHER STARTED *******".bold().bright_cyan().underline()
//     );

//     fs::create_dir("Download").unwrap_or_else(|why| {
//         println!("! {:?}", why);
//     });
//     let timer_start = Instant::now();
//     // ? does the thing only if there is no error

//     // Get all the urls from the file :D and save it into a vector of type string
//     let paths: Vec<String> = read_lines("urls.txt")?;
//     let client = Client::builder().build()?;


//     // Iterate over the urls
//     let fetches = futures::stream::iter(
//     paths.into_iter().map(|path| {

//         // get(path) sends HTTP GET request and waits for the request to be finished
//         let send_fut = client.get(&path).send();
//         // fn find_alt(alt: &str){

//         //     let  alt = alt.to_string();
//         //     // let get =   ;
//         // }
   
        
//         async move {    
            
//             // print
//             // println!("SENDING REQUEST TO : {}", path);


//             // OLD CODE USED THIS AND DIDNT HAVE 
//             //"send_fut" or "client" match reqwest::get(&path).await {
//             match send_fut.await {
//                 // OK() is executed if the request is succesful
//                 Ok(resp) => {

//                     // Prints


//                     // println!(" GOT RESPONSE");
//                     // println!(" WAITING FOR TEXT ");
//                     // Wait for the response text
//                     match resp.text().await {
//                         // If the response text is ok
//                         Ok(text) => {
//                             // Print bytes, modify to save file
                            
//                             // println!("RESPONSE: {} bytes from {}", text.len(), path);
                            

//                             // Create a progress bar
                            
//                             // let total_size = 1232;
//                             // let pb = ProgressBar::new(total_size);
//                             // pb.set_style(ProgressStyle::default_bar()
//                             // .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
//                             // .progress_chars("#>-"));
//                             // pb.set_message(&format!("Downloading  file {}", mb));

//                             let url: &str = path.as_str();

//                             let mut file_name = url.split('/').last().unwrap_or("unknown");
//                             let file_type = file_name.split('.').last().unwrap_or("unknown");

//                             // if file_type == ""{
//                             //     path.pop();
//                             //     file_name = path.as_str().split('/').last().unwrap_or("unknown");
//                             // }
//                             let file_path = format!("Download/{}", file_name);
                            
//                             // let download_path = "downloads.dir";
//                             let _ = match get_table(url) {
//                                 Ok(Some(table)) => Ok(println!("The table  is: {}", table)),
//                                 Ok(None) => Ok(println!("No table found")),
//                                 Err(e) => Err(e),
//                             };

//                             let _ = download_file(text, file_name, file_type, &file_path).await;
//                             println!("{} | {}",
//                                 "File downloaded".yellow().underline(), file_path.italic()
//                             );

//                         }
//                         // In case of error in the response text
//                         Err(_) => println!("ERROR reading {}", path),
//                     }
//                 }
//                 //Err() is executed if the the request is unsuccessful
//                 Err(_) => println!("ERROR downloading {}", path),
//             }
//         }
//     })
//     // After iterating over the urls    
//     ).buffer_unordered(50).collect::<Vec<()>>();
//     fetches.await;
//     let duration = timer_start.elapsed();
//     println!("{} | {:?}",
//         "Time Taken".bright_magenta(),duration
//     );
//     Ok(())

// }

// Purana code khatam hai
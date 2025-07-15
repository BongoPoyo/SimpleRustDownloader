This is a simple downloader for websites

# Usage

- Run the Application and enter a url like this.
```

https://dl.chughtailibrary.com/files/repository/book_quest/natura_science/1007/
```


# Installation
Go to the releases tag to get compiled binaries.
- Windows
```

wget https://github.com/BongoPoyo/SimpleRustDownloader/releases/latest/download/win-x86_64.zip
```
- Linux
```

wget https://github.com/BongoPoyo/SimpleRustDownloader/releases/latest/download/linux-x86_64.zip
```


# GUI
- By default it will run the gui.

# CLI 
- Pass the cli cmd-line-arg to run the CLI version.
```binary --cli```

- Other command line arguments.
```
            --cli or -c                     Runs cli
            --gui or -g                     Runs gui(Default: Runs gui instead of cli.) 
            --help or -h                    Shows this help form
            --url <url> or -u <url>         Used to set the url
            --debug or -d                   Displays debug info
            --overrride or -o               Overrides the pre-existing files with the new one (Default: Skips re-downloading the pre-existing files.)
```


# Notes
- Running the application once will create `Download` directory and the application will only use this directory, for pdfs, imgs and folders.
- The pdf converter only converts files inside `pdf_images`
- Open download location will open the location of last file.

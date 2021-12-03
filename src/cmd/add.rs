/*
todo:
1. Check entered filepath is correct
        - Make sure it ends *without* a slash it must end in a file-location
        - If filepath is pointing to a directory only inform user and exit
2. Filepath is correct
        - split filepath into filename and directory
3. Check if directory exists inside content dir
        - if it does not create it now
4. Check if file exists in above verified directory
        - if it does inform user and exit (possibly prompt to change it?)
        - if it does not create file with front matter header
5. Check if --file-title flag is passed after file is created
    -- Add it to the file front matter
*/

use std::path::Path;
use std::env::current_dir;

use errors::Result;

use crate::console;
use utils::fs::create_directory;

pub fn add(file_path: Option<&Path>, file_title: Option<&str>) -> Result<()> {
    // @FIXME safe to unwrap here?
    let mut current_dir = current_dir().unwrap();
    current_dir.push("content"); // gets current directory and appends content to it

    if current_dir.exists() { // checks if the __dir/content directory exists all zola projects have it

        // @FIXME safe to unwrap these here?
        current_dir.push(file_path.unwrap()); // append the users given path to our content directory
        // @TODO do I need an os str?
        let path_as_string = &current_dir.as_os_str().to_str().unwrap(); // path as string to check last character to determine if file or dir

        // @FIXME how to handle string, is_dir/is_file checks the hard disk I need to check it before its
        // created
        // IDEA, couldnt we just check if the file exists with the given path ? If not create the
        // directories etc anyway if it does then its dir exists? hmmm
        if path_as_string.ends_with("/") || path_as_string.ends_with("\\") { // file path needs to point to a file, not a directory.
            println!("generated path from input: \n {:?}", current_dir);
            console::error("Path given is a directory, please use a path to a file.");
        } else {
            println!("path is a file");
            println!("Lets create it...");

            if current_dir.extension().is_none() { // checks if given path has an extension like .js/svg/png etc
                current_dir.set_extension("md");
            }

            let path = current_dir.as_path();
            println!("{:?}", path);
            // let res = create_directory(path);
            // if res.is_err() {
            //     println!("{:?}", res)
            // }

            //@todo create newly created files front matter if its markdown check utils::fs
            //@todo if file_title and extension is md edit front matter to include title
        }
    } else {
        console::error("Content directory does not exist, ensure this is a zola project.");
        ::std::process::exit(1);
    }

    Ok(())
}

// #[cfg(windows)]
// use std::os::windows::ffi::OsStrExt;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use std::io::ErrorKind;
use std::path::Path;
use std::env::current_dir;
use std::fs::create_dir;

use errors::Result;

use crate::console;
use utils::fs::create_file;

// #[cfg(windows)]
// fn has_trailing_slash(p: &Path) -> bool {
//     let last = p.as_os_str().encode_wide().last();
//     last == Some(b'\\' as u16) || last == Some(b'/' as u16)
// }

#[cfg(unix)]
fn has_trailing_slash(p: &Path) -> bool {
    p.as_os_str().as_bytes().last() == Some(&b'/')
}

fn validate_users_directory() -> bool {
    if let Err(_dir) = current_dir() {
        match _dir.kind() {
            ErrorKind::NotFound => console::error("Directory not found, does it exist?"),
            ErrorKind::PermissionDenied => console::error("Permissions denied in current directory."),
            _ => console::error("failed to validate current directory")
        }
        return false;
    }

    true
}

const _INDEX_CONTENT: &str = r#"+++
title = ""
template = ""
page_template = ""
sort_by = ""
+++
"#;

const FILE_CONTENT: &str = r#"
description=""
date=
+++
"#;

fn custom_title(title: &str) -> String {
    let mut front_matter = String::from("+++\ntitle=");
    front_matter.push_str(title);
    front_matter.push_str(FILE_CONTENT);
    front_matter
}

pub fn add(file_path: Option<&Path>, file_title: Option<&str>) -> Result<()> {

    // run validate_current_dir to check if directory is safe then we can use ? on
    // current_dir safely, seems to be tricky as its io::Result matching is problematic
    if validate_users_directory() {
        let mut user_dir = current_dir()?;
        user_dir.push("content"); // point the directory(zola project) to the content folder

        if user_dir.exists() { // checks if the __dir/content directory exists all zola projects have it

            if let Some(file_path) = file_path { // I am verifying here again but cli will complain if path not passed

                if has_trailing_slash(file_path) { // file path needs to point to a file, not a directory.
                    console::error("Path given is a directory, please use a path to a file.");
                } else {
                    let mut file_path_buf = file_path.to_path_buf();

                    // should I do this we no longer use the path since we converted it to mutable
                    // pathbuf
                    drop(file_path);

                    // check if the given file path has an extension if not default to md
                    if file_path_buf.extension().is_none() {
                        file_path_buf.set_extension("md");
                    }

                    /*
                        using the current ../content/ path, map over each component of the
                        user passed file path, append to current_dir and check if the directory exists
                        if not create it and a _index.md file move on to the next component, if component
                        has an extension create the file with front matter and end loop

                        To stop weird behaviour and users being funny, stop loop at the first component with an
                        extension
                     */

                    for segment in file_path_buf.components() {
                        user_dir.push(&segment);
                        if let Some(ext) = user_dir.extension() {
                            console::info(format!("Creating {:?} file.", &segment.as_os_str()).as_str());
                            if ext == "md" {
                                if let Some(title) = file_title {
                                    return create_file(&user_dir, &custom_title(title))
                                } else {
                                    return create_file(&user_dir, &custom_title("\"\""))
                                }
                            } else {
                                return create_file(&user_dir, "")
                            }
                        } else {
                            if !user_dir.is_dir() {
                                console::info("Directory does not exist:");
                                console::info(format!("Creating {:?} and adding _index file.", &user_dir).as_str());
                                create_dir(&user_dir)?;
                                create_file(&user_dir.join("_index.md"), _INDEX_CONTENT)?;
                            }
                        }
                    }
                }
            }
        } else {
            console::error("Content directory does not exist, ensure this is a zola project.");
            ::std::process::exit(1);
        }

        Ok(())
    } else {
        ::std::process::exit(1);
    }
}
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

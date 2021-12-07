#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;

use std::ffi::OsStr;
use std::io::ErrorKind;
use std::path::Path;
use std::env::current_dir;
use std::fs::create_dir;

use errors::Result;

use crate::console;
use crate::prompt::ask_bool;

use utils::fs::create_file;

#[cfg(windows)]
fn has_trailing_slash(p: &Path) -> bool {
    let last = p.as_os_str().encode_wide().last();
    last == Some(b'\\' as u16) || last == Some(b'/' as u16)
}

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

fn create_project_file(path:&Path, ext:&OsStr, file_title:Option<&str>) -> Result<()> {

        if ext == "md" {
            if let Some(title) = file_title {
                create_file(&path, &custom_title(title))?
            } else {
                return create_file(&path, &custom_title("\"\""))
            }
        } else {
            return create_file(&path, "")
        }

    Ok(())
}

fn create_project_dir(path: &Path) -> Result<()> {
        if !path.is_dir() {
            create_dir(&path)?;
            create_file(&path.join("_index.md"), _INDEX_CONTENT)?;
        }

    Ok(())
}

pub fn add(file_path: Option<&Path>, file_title: Option<&str>) -> Result<()> {
    let mut dest_path = file_path.unwrap().to_path_buf();

    // run validate_current_dir to check if directory is safe then we can use ? on
    // current_dir safely, seems to be tricky as its io::Result matching is problematic
    if validate_users_directory() {
        let mut user_dir = current_dir()?;

        user_dir.push("content"); // point the directory(zola project) to the content folder

        if user_dir.exists() { // checks if the ../content directory exists all zola projects have it

            if has_trailing_slash(&dest_path) { // file path needs to point to a file, not a directory.
                console::error("Path given is a directory, please use a path to a file.");
            } else {
                // check if the given file path has an extension if not default to md
                if dest_path.extension().is_none() {
                    dest_path.set_extension("md");
                }

                let mut segments = dest_path.components().peekable(); // create peekable so we can check if last item
                while let Some(segment) = segments.next() {
                    user_dir.push(&segment);

                    if segments.peek().is_some() {
                        // still more path to go through

                        if let Some(ext) = user_dir.extension() {
                            // This segment has a extension lets check if its the end of the path or
                            // if its got a trailing slash
                            console::warn("Warning: file extension detected in entered path, is this intended?");
                            let valid_entry = ask_bool(">", false)?;

                            if valid_entry {
                                console::warn("Notice: intended file extension detected in entered path, ignoring remaining path\n");
                                console::success("created content at path:");
                                console::info(format!("{:?}", &user_dir.as_os_str()).as_str());
                                return create_project_file(&user_dir, &ext, file_title);
                            }
                        } else {
                            create_project_dir(&user_dir)?
                        }

                    } else {
                        create_project_file(&user_dir, user_dir.extension().unwrap(), file_title)?;
                        console::success("created content at path:");
                        console::info(format!("{:?}", &user_dir.as_os_str()).as_str());
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

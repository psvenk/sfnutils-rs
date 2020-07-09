use std::{env, fs, io, process, str};
use std::ffi::OsString;
use std::collections::HashMap;
use std::iter::Iterator;
use std::io::Error;
use std::fmt;

const MAX_FILES: usize = 100;

type NameMap = HashMap<String, u8>;

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct ShortName {
    name: [u8; 8],
    ext: [u8; 3],
}

impl ShortName {
    fn from_str(orig_name: &str, map: &mut NameMap) -> Self {
        let mut name = orig_name.as_bytes().to_vec();
        let mut remove_i_list = Vec::with_capacity(name.len());
        let mut separator_i = None;
        let mut modified = false;

        for (i, x) in name.iter_mut().enumerate().rev() {
            if !x.is_ascii() || *x == b'+' {
                modified = true;
                *x = b'_';
            } else if *x == b'.' && i > 0 && separator_i == None {
                separator_i = Some(i);
            } else if *x == b' ' || *x == b'.' {
                modified = true;
                remove_i_list.push(i);
            } else {
                *x = x.to_ascii_uppercase();
            };
        }

        let mut ext;
        match separator_i {
            Some(i) => {
                for j in &remove_i_list {
                    if *j > i {
                        name.remove(*j);
                    }
                }
                name.remove(i);
                ext = name.split_off(i);
                if ext.len() > 3 {
                    modified = true;
                }
                ext.truncate(3);
            }
            None => ext = Vec::new(),
        };

        if name.len() > 8 {
            modified = true;
        }

        for i in &remove_i_list {
            if *i < name.len() {
                name.remove(*i);
            }
        }

        if modified {
            if name.len() > 6 {
                name.truncate(6);
            }

            // Rationale: all non-ASCII characters have been stripped.
            let key = unsafe {
                String::from_utf8_unchecked(name.clone())
            };
            let num = match map.get(&key) {
                Some(n) => n + 1,
                None => 1,
            };
            map.insert(key, num);

            if num < 10 {
                name.extend_from_slice(&[
                    b'~',
                    b'0' + num,
                ]);
            } else {
                name.remove(name.len() - 1);
                name.extend_from_slice(&[
                    b'~',
                    b'0' + num / 10,
                    b'0' + num % 10,
                ]);
            }
        }

        let mut name_array = [b' '; 8];
        let mut ext_array = [b' '; 3];
        for (i, x) in name_array.iter_mut().enumerate() {
            if let Some(c) = name.get(i) {
                *x = *c;
            }
        }
        for (i, x) in ext_array.iter_mut().enumerate() {
            if let Some(c) = ext.get(i) {
                *x = *c;
            }
        }
        ShortName { name: name_array, ext: ext_array }
    }
}

impl fmt::Display for ShortName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = str::from_utf8(&self.name).map_err(|_| fmt::Error)?;
        let ext = str::from_utf8(&self.ext).map_err(|_| fmt::Error)?;
        write!(f, "{} {}", name, ext)
    }
}

fn fatal(err: Error) -> ! {
    let process_name = match env::args_os().nth(0) {
        Some(os_str) => {
            match os_str.into_string() {
                Ok(s) => s,
                Err(_) => "Error".to_string(),
            }
        }
        None => "Error".to_string()
    };
    eprintln!("{}: {}", process_name, err);
    process::exit(1);
}

fn main() {
    let mut files;
    match getfiles(&match env::args_os().nth(1) {
        Some(s) => s,
        None => OsString::from("."),
    }, MAX_FILES) {
        Ok(x) => files = x,
        Err(err) => fatal(err),
    }

    files.sort_unstable();
    for file in files {
        println!("{}", file);
    }
}

fn getfiles(path: &OsString, max_files: usize)
    -> io::Result<Vec<ShortName>>
{
    let mut files = Vec::new();
    let mut map = HashMap::new();

    for entry in fs::read_dir(path)? {
        if files.len() >= max_files {
            break;
        }
        let name_os_string = entry?.file_name();
        let name = name_os_string.to_string_lossy();
        files.push(ShortName::from_str(&name, &mut map));
    }
    Ok(files)
}

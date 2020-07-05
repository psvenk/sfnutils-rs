use std::{env, fs, io, ffi};
use std::collections::HashMap;
use std::iter::Iterator;

type NameMap = HashMap<String, u8>;

struct ShortName {
    name: Vec<u8>, // TODO enforce size 8
    ext: Vec<u8>, // TODO enforce size 3
}

impl ShortName {
    fn from_string(orig_name: &str, map: &mut NameMap) -> Self {
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

        for i in remove_i_list {
            name.remove(i);
        }

        if modified {
            if name.len() > 6 {
                name.truncate(6);
            }

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

        ShortName { name: name, ext: ext }
    }
}

impl From<ShortName> for String {
    fn from(sn: ShortName) -> Self {
        let name = unsafe {
            String::from_utf8_unchecked(sn.name)
        };
        let ext = unsafe {
            String::from_utf8_unchecked(sn.ext)
        };
        format!("{:8} {:3}", name, ext)
    }
}

// Convert an iterator of u8 to a fixed-size array
// fn bytes_to_array<A: Iterator + Default, T: Iterator>(bytes: T) -> A {
//     let mut arr = A::default();
//     for (i, _) in arr.enumerate() {
//         match bytes.next() {
//             Some(n) => arr[i] = n,
//             None => arr[i] = b' ',
//         }
//     }
//     arr
// }

const MAX_FILES: u8 = 100;

fn main() {
    let num_files = getfiles(match env::args_os().nth(1) {
        Some(s) => s,
        None => ffi::OsString::from("."),
    }, MAX_FILES).unwrap();
}

fn getfiles(path: ffi::OsString, max_files: u8) -> io::Result<u8> {
    let mut num_files = 0;
    let mut map = HashMap::new();

    for entry in fs::read_dir(path)? {
        if num_files >= max_files {
            break;
        }

        let name_os_string = entry?.file_name();
        let name = name_os_string.to_string_lossy();
        // println!("{}", name);
        let shortname = ShortName::from_string(&name, &mut map);
        println!("{}", String::from(shortname));

        num_files += 1;
    }
    Ok(0)
}

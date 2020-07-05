use std::{env, fs, io, ffi};
use std::collections::HashMap;
use std::iter::Iterator;

struct NameMap<'a>(HashMap<&'a str, u8>);

struct ShortName<'a> {
    name: &'a [u8],
    ext: &'a [u8],
}

impl ShortName<'_> {
    fn from_string<'a>(orig_name: &'a str, map: &NameMap) -> Self {
        let mut name: [u8] = orig_name.as_bytes();
        let mut ext: [u8] = orig_name.as_bytes();
        for i in 0..8 {
            // TODO do whatever transformations
        }
        // TODO trim down arrays
        // Actually maybe use the String method truncate or something like that?
        // We still need to fill it to 8 characters / 3 characters using spaces
        // name.copy_from_slice(&orig_name.bytes().take(8).collect::<Vec<_>>());
        // ext.copy_from_slice(&orig_name.bytes().take(3).collect::<Vec<_>>());
        // let name = &'a orig_name.bytes().take(8).collect::<Vec<_>>();
        // let ext  = &'a orig_name.bytes().take(3).collect::<Vec<_>>();
        // ShortName { name: &name, ext: &ext }
        ShortName { name: [65, 66, 67, 68, 69, 70, 71, 72], ext: [66; 3] }
    }
}

impl From<ShortName<'_>> for String {
    fn from(sn: ShortName) -> Self {
        let name = String::from_utf8(sn.name.to_vec()).unwrap();
        let ext = String::from_utf8(sn.ext.to_vec()).unwrap();
        format!("{}.{}", name, ext)
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
    let map = NameMap(HashMap::new());

    for entry in fs::read_dir(path)? {
        if num_files >= max_files {
            break;
        }

        let name_os_string = entry?.file_name();
        let name = name_os_string.to_string_lossy();
        // println!("{}", name);
        let shortname = ShortName::from_string(&name, &map);
        println!("{}", String::from(shortname));

        num_files += 1;
    }
    Ok(0)
}

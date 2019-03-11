use std::fs::File;
use std::io::{Read, Result, Error, ErrorKind};
use std::path::Path;


#[cfg(not(target_arch = "wasm32"))]
/// Read a file at `path` into a String
pub fn read_file(path: &Path) -> Result<String> {
    info!("Loading file {:?}", path);
    let mut f = File::open(&path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}

#[cfg(target_arch = "wasm32")]
/// Read a file at `path` into a String
pub fn read_file(path: &Path) -> Result<String> {
    let key = path.to_str().unwrap();
    let window = stdweb::web::window();
    let storage = window.local_storage();
    console!(log, key);
    let ret = storage
        .get(key)
        .map(|i|base64::decode(&i).unwrap())
        .map(|i|std::str::from_utf8(&i).unwrap().to_owned())
        .ok_or(Error::new(ErrorKind::NotFound, "not found in local storage"));

    console!(log, format!("{:?}", ret));

    ret
}
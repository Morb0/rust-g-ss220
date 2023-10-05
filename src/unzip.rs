use crate::{error::Result, http::HTTP_CLIENT, jobs};
use reqwest::blocking::RequestBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;
use zip::ZipArchive;

struct UnzipPrep {
    req: RequestBuilder,
    unzip_directory: String,
}

fn construct_unzip(url: &str, unzip_directory: &str) -> UnzipPrep {
    let mut reqh = None;
    HTTP_CLIENT.with(|client| {
        reqh = Some(client.borrow_mut().as_ref().unwrap().get(url));
    });
 
    let dir_copy = unzip_directory.to_string();
    let req = reqh.unwrap();

    UnzipPrep {
        req,
        unzip_directory: dir_copy,
    }
}

byond_fn!(fn unzip_download_async(url, unzip_directory) {
    let unzip = construct_unzip(url, unzip_directory);
    Some(jobs::start(move ||
        do_unzip_download(unzip).unwrap_or_else(|e| e.to_string())
    ))
});

fn do_unzip_download(prep: UnzipPrep) -> Result<String> {
    let unzip_path = Path::new(&prep.unzip_directory);
    let response = prep.req.send()?;

    let content = response.bytes()?;

    let reader = std::io::Cursor::new(content);
    let mut archive = ZipArchive::new(reader)?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;

        let file_path = unzip_path.join(entry.name());

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?
        }

        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&file_path)?;

        let mut writer = std::io::BufWriter::new(file);
        std::io::copy(&mut entry, &mut writer)?;
        writer.flush()?;
    }

    Ok("true".to_string())
}

byond_fn!(fn unzip_check(id) {
    Some(jobs::check(id))
});

// Copyright (c) 2022 Lorenzo Carbonell <a.k.a. atareao>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use async_zip::base::read::seek::ZipFileReader; use chrono::Datelike;
//AsyncReadExt
use log::{debug, error, info};
use spinners::{Spinner, Spinners};
use std::{io::Cursor, path::Path, path::PathBuf, error::Error};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use std::collections::HashMap;
use minijinja::{context, Environment};
use tokio::{
    fs::{create_dir_all, File, OpenOptions},
    io::BufReader,
};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use super::variable::Variable;

pub async fn fetch_url(url: &str, filename: &str) -> Result<()> {
    info!("fetch_url");
    let mut spinner = Spinner::new(Spinners::Dots9, "Downloading templates".into());
    let response = reqwest::get(url).await?;
    spinner.stop_and_persist("✔", "Downloaded!".into());
    let mut file = std::fs::File::create(filename)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

pub async fn unzip(zipfile: &str, out_dir: &PathBuf) -> Result<()> {
    info!("unzip");
    //let file = File::open(zipfile).await?;
    let out_dir = Path::new(out_dir);
    let file = File::open(zipfile).await?;
    let buf_reader = BufReader::new(file).compat();
    let mut reader = ZipFileReader::new(buf_reader).await?;
    for index in 0..reader.file().entries().len() {
        let entry = reader.file().entries().get(index).unwrap();
        let path = out_dir.join(sanitize_file_path(entry.filename().as_str().unwrap()));
        // If the filename of the entry ends with '/', it is treated as a directory.
        // This is implemented by previous versions of this crate and the Python Standard Library.
        // https://docs.rs/async_zip/0.0.8/src/async_zip/read/mod.rs.html#63-65
        // https://github.com/python/cpython/blob/820ef62833bd2d84a141adedd9a05998595d6b6d/Lib/zipfile.py#L528
        let entry_is_dir = entry.dir().unwrap();

        let mut entry_reader = reader.reader_without_entry(index).await?;

        if entry_is_dir {
            // The directory may have been created if iteration is out of order.
            if !path.exists() {
                debug!("{:?}", &path);
                create_dir_all(&path).await?;
            }
        } else {
            // Creates parent directories. They may not exist if iteration is out of order
            // or the archive does not contain directory entries.
            let parent = path.parent().ok_or(Box::from("A file entry should have parent directories") as Box<dyn Error + Send + Sync>)?;
            if !parent.is_dir() {
                create_dir_all(parent).await?;//.expect("Failed to create parent directories");
            }
            let writer = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .await?;
                //.expect("Failed to create extracted file");
            futures_lite::io::copy(&mut entry_reader, &mut writer.compat_write())
                .await?;
                //.expect("Failed to copy to extracted file");

            // Closes the file and manipulates its metadata here if you wish to preserve its metadata from the archive.
        }
    }


    Ok(())
}

/// Returns a relative path without reserved names, redundant separators, ".", or "..".
fn sanitize_file_path(path: &str) -> PathBuf {
    // Replaces backwards slashes
    path.replace('\\', "/")
        // Sanitizes each component
        .split('/')
        .map(sanitize_filename::sanitize)
        .collect()
}


pub fn reder_template(vars: &Vec<Variable>, jinja_file: &PathBuf, jinja_content: &str) -> Result<String> {
    let mut var = HashMap::new();
    for item in vars{
        var.insert(&item.key, &item.value);
    }
    let now = chrono::Utc::now();
    let key = &"YEAR".to_string();
    let value = &format!("{}", now.year());
    var.insert(key, value);
    let key = &"MONTH".to_string();
    let value = &format!("{}", now.month());
    var.insert(key, value);
    let key = &"DAY".to_string();
    let value = &format!("{}", now.day());
    var.insert(key, value);
    let mut environment = Environment::new();
    let file = jinja_file.as_os_str().to_str().unwrap();
    environment.add_template(file, jinja_content)?;

    let template = environment.get_template(file)?;
    let ctx = context!(var);

    Ok(template.render(&ctx)?)
}

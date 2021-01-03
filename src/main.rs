pub mod commander;
use std::future::Future;
use std::marker::Send;
use std::pin::Pin;

use commander::{ Command, Commands };
use serde_json::{Result, Value, Map};

use std::fs::{File, DirEntry, DirBuilder};
use std::io::prelude::*;


const REGISTRY: &str = "https://registry010.theboys619.repl.co";
const API: &str = "https://registry010.theboys619.repl.co/api";
const PACKAGE: &str = "https://registry010.theboys619.repl.co/packages";

async fn get_body(url: &str) -> String {
  match reqwest::get(url).await {
    Ok(response) => {
      match response.text().await {
        Ok(text) => return text,
        Err(_) => println!("Fail!")
      }
    }
    Err(err) => println!("{}", err)
  }

  return "".to_string();
}

fn writeFiles(data: Vec<Value>) -> Pin<Box<dyn Future<Output = ()>>> {
  Box::pin(async move {
    for item in data {
      let obj: &Map<String, Value> = item.as_object().unwrap();
      let path = obj["path"].as_str().unwrap().to_owned() + "/" + obj["name"].as_str().unwrap(); // Get path of object
  
      if obj["type"].as_str().unwrap() == "Directory" {
        let mut dirb = DirBuilder::new();
        dirb.recursive(true).create(path.clone()); // recursively create the path for folder that might not exist

        println!("Wrote Folder: {}", path.clone());

        let filesList: Vec<Value> = obj["files"].as_array().unwrap().to_vec(); // Get filelist from directory
        writeFiles(filesList).await; // read those files/folders and write them
      } else if obj["type"].as_str().unwrap() == "File" {
        let filec = File::create(path.clone()); // create a file
        if filec.is_ok() { // check if it is ok
          let mut file = filec.unwrap(); // get the file

          let mut url: String = REGISTRY.to_owned(); // create package URL to get data of the file
          url += "/";
          url += &path;
  
          let filedata: String = get_body(&*url).await; // make the request
          file.write_all(filedata.as_bytes()); // write the file to the package folder

          println!("Wrote File: {}", path.clone());
        } else {
          println!("Could not write file: {}", path.clone());
        }
      }
    }
  })
}

async fn install(args: Vec<String>) -> () {
  let pkgname: String = args[0].clone();
  let mut url: String = PACKAGE.to_owned();
  url += "/";
  url += &pkgname;

  let body: String = get_body(&*url).await; // make request to registry to get file list
  let resdata: Value = serde_json::from_str(&*body).unwrap(); // unwrap the response
  let data: Vec<Value> = resdata.as_array().unwrap().to_vec(); // get the data as an array then as a vector

  writeFiles(data).await; // write all the files/folders from the file list
  println!("Wrote package {}", pkgname);
}

fn publish(args: Vec<String>) {
  println!("Ok");
}

#[tokio::main]
async fn main() {
  let mut parser = Commands::new("apm");

  parser
    .command("install <package>")
    .description("Install a package")
    .asyncaction(&install);

  parser
    .command("publish <package>")
    .description("Publish a package")
    .action(&publish);

  // if argc < 2 {
  //   help();
  // }

  parser.parse(std::env::args().collect()).await;
}

#[macro_use] extern crate rocket;

use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::serde::{self, Deserialize, Serialize};
//use rocket::tokio::fs;
use std::fs;
use std::path::PathBuf;
use rocket::serde::json::Json;


#[derive(Serialize)]
struct  FileList{
    files:Vec<String>
}

#[derive(FromForm)]
struct Upload<'r>{
    file:TempFile<'r>
}

#[post("/upload",data="<form>")]
async fn upload(mut form:Form<Upload<'_>>)->String{
    let fileName = form.file.name().unwrap_or("upload_bin").to_string();
    //create Upload Folder if it doesn't Exists

    let folder = PathBuf::from("uploads");
    rocket::tokio::fs::create_dir_all(&folder).await.unwrap();

    let filepath = folder.join(&fileName);

    //save the uploaded file

    if let Err(e) = form.file.persist_to(&filepath).await{
        return  format!(" Failed to save the file {}",e);
    }

    format!("File uploaded as {}", fileName)


}

#[get("/files")]
async fn list_files()->Json<FileList>{
    let path  = std::path::Path::new("uploads");
    let mut files = vec![];

    if let Ok(entries) = std::fs::read_dir(path){
        for entry in entries{
            if let Some(name) = entry.unwrap().file_name().to_str(){
                files.push(name.to_string());
            }
        }
    }
    Json(FileList{files})
}

#[launch]
fn rocket()->_{
    rocket::build().mount("/", routes![upload,list_files])
}


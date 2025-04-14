
#[macro_use] extern crate rocket;

use rocket::fs::TempFile;
use rocket::form::Form;
use rocket::tokio::fs;
use std::path::PathBuf;

#[derive(FromForm)]
struct Upload<'r>{
    file:TempFile<'r>
}

#[post("/upload",data="<form>")]
async fn upload(mut form:Form<Upload<'_>>)->String{
    let fileName = form.file.name().unwrap_or("upload_bin").to_string();
    //create Upload Folder if it doesn't Exists

    let folder = PathBuf::from("uploads");
    fs::create_dir_all(&folder).await.unwrap();

    let filepath = folder.join(&fileName);

    //save the uploaded file

    if let Err(e) = form.file.persist_to(&filepath).await{
        return  format!(" Failed to save the file {}",e);
    }

    format!("File uploaded as {}", fileName)


}


#[launch]
fn rocket()->_{
    rocket::build().mount("/", routes![upload])
}


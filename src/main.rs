extern crate zip;
use std::env;
use std::env::Args;
use std::fs;
// use zip::read;

fn main() {
    let mut args: Args = env::args();
    println!("{:?}", args);
    args.next();
    list_zips_from_folder(args.next().expect("sem folder"));
}

#[derive(Debug)]
enum FolderSubItem {
    OtherFile,
    SubFolder,
    Zip(String),
}

fn get_folder_info(folder: &str) -> Vec<FolderSubItem> {
    // println!("\n -> {}", folder);
    let iter = fs::read_dir(folder).expect("erro da leitura do diretorio");
    // println!("{:#?}", iter);
    let mut vec = Vec::new();
    for entry in iter {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_dir() {
            vec.push(FolderSubItem::SubFolder);
        } else {
            if entry.path().extension().unwrap() == "zip" {
                vec.push(FolderSubItem::Zip(
                    entry.path().to_str().unwrap().clone().to_string(),
                ));
            } else {
                vec.push(FolderSubItem::OtherFile);
            }
        }
    }
    vec
}

fn list_zips_from_folder(folder: String) {
    let iter = fs::read_dir(folder);
    let iter = match iter {
        Ok(n) => n,
        _ => return,
    };
    for entry in iter {
        let entry = entry.expect("tira de dentro");
        if !entry.file_type().expect("msg").is_dir() {
            continue;
        }
        let sub_list = get_folder_info(&entry.path().to_str().unwrap());

        if sub_list.iter().any(|i| is_it_a_subfolder(&i) == true) {
            continue;
        }

        let sub_list_zips: Vec<&FolderSubItem> = sub_list
            .iter()
            .filter(|z| is_it_a_zipfile(&z) == true)
            .collect();

        for z in sub_list_zips.iter() {
            let string = match z { FolderSubItem::Zip(a) => a, _ => continue };
            println!("{:?}", string);
            
        }
    }
}

fn is_it_a_subfolder(sub_folder_item: &FolderSubItem) -> bool {
    match sub_folder_item {
        FolderSubItem::OtherFile => false,
        FolderSubItem::Zip(_) => false,
        FolderSubItem::SubFolder => true,
    }
}

fn is_it_a_zipfile(sub_folder_item: &FolderSubItem) -> bool {
    match sub_folder_item {
        FolderSubItem::OtherFile => false,
        FolderSubItem::Zip(_) => true,
        FolderSubItem::SubFolder => false,
    }
}

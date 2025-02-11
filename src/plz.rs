use crate::mapper::MapperDE;
use crate::mapper::MapperOrt;
use dotenv::dotenv;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
 Author Gaurav Sablok
 SLB Potsdam
 Date: 2025-2-11
*/

pub fn plz_mapper(plz: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let file1 = std::env::var("file1").expect("file not present");
    let file2 = std::env::var("file2").expect("file not present");
    let file1read = File::open(file1).expect("file not present");
    let file2read = File::open(file2).expect("file not present");
    let file1read = BufReader::new(file1read);
    let file2read = BufReader::new(file2read);

    let mut mapper_de: Vec<MapperDE> = Vec::new();
    let mut mapper_ort: Vec<MapperOrt> = Vec::new();

    for i in file1read.lines() {
        let line = i.expect("file not found");
        if line.starts_with("plz") {
            continue;
        } else if !line.starts_with("plz") {
            let mapper: Vec<_> = line.split(",").collect::<Vec<_>>();
            if mapper.len() == 6usize {
                mapper_de.push(MapperDE {
                    plz: mapper[0].to_string(),
                    note: mapper[1].to_string(),
                    einwohner: mapper[2].replace("/", "-").to_string(),
                    qkm: mapper[3].to_string(),
                    lat: mapper[4].to_string(),
                    lon: mapper[5].to_string(),
                });
            }
        }
    }

    for i in file2read.lines() {
        let line = i.expect("file not found");
        if line.starts_with("osm") {
            continue;
        } else if !line.starts_with("osm") {
            let ort_mapper: Vec<_> = line.split(",").collect::<Vec<_>>();
            mapper_ort.push(MapperOrt {
                osmid: ort_mapper[0].to_string(),
                ags: ort_mapper[1].to_string(),
                ord: ort_mapper[2].replace(" ", "-").to_string(),
                plz: ort_mapper[3].to_string(),
                landries: ort_mapper[4].replace(" ", "-").to_string(),
                bundesland: ort_mapper[5].replace(" ", "-").to_string(),
            })
        }
    }

    let mut searched_plz1: Vec<MapperDE> = Vec::new();
    let mut searched_plz2: Vec<MapperOrt> = Vec::new();
    for i in mapper_de.iter() {
        if i.plz == plz.to_string() {
            searched_plz1.push(MapperDE {
                plz: i.plz.clone(),
                note: i.note.clone(),
                einwohner: i.einwohner.clone(),
                qkm: i.qkm.clone(),
                lat: i.lat.clone(),
                lon: i.lon.clone(),
            })
        }
    }
    for i in mapper_ort.iter() {
        if i.plz == plz.to_string() {
            searched_plz2.push(MapperOrt {
                plz: i.plz.clone(),
                osmid: i.osmid.clone(),
                ags: i.ags.clone(),
                ord: i.ord.clone(),
                landries: i.landries.clone(),
                bundesland: i.bundesland.clone(),
            });
        }
    }

    for i in searched_plz1.iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            i.plz, i.note, i.einwohner, i.qkm, i.lat, i.lon
        );
    }
    for j in searched_plz2.iter() {
        println!(
            "{}\t{}\t{}\t{}\t{}\t{}",
            j.plz, j.osmid, j.ags, j.ord, j.landries, j.bundesland
        );
    }

    Ok("The searched results are as follows".to_string())
}

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

pub fn osm_mapper(osm: &str) -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    let file2 = std::env::var("file2").expect("file not present");
    let file2read = File::open(file2).expect("file not present");
    let file2read = BufReader::new(file2read);

    let mut mapper_ort: Vec<MapperOrt> = Vec::new();

    for i in file2read.lines() {
        let line = i.expect("file not found");
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

    let mut searched_plz2: Vec<MapperOrt> = Vec::new();
    for i in mapper_ort.iter() {
        if i.osmid == osm.to_string() {
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

    Ok("The searched results are as follows".to_string())
}

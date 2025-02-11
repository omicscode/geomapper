mod ags;
mod args;
mod bundesland;
mod einwohner;
mod landkries;
mod lat;
mod lon;
mod mapper;
mod note;
mod ord;
mod osm;
mod plz;
mod qkm;
use crate::ags::ags_mapper;
use crate::args::CommandParse;
use crate::args::Commands;
use crate::bundesland::bundesland_mapper;
use crate::einwohner::einwohner_mapper;
use crate::landkries::landkries_mapper;
use crate::lat::latitude_mapper;
use crate::lon::longitude_mapper;
use crate::note::note_mapper;
use crate::ord::ord_mapper;
use crate::osm::osm_mapper;
use crate::plz::plz_mapper;
use crate::qkm::qkm_mapper;
use clap::Parser;
/*
* Author Gaurav Sablok
* SLB Potsdam
  Date: 2025-2-11
*
* a geospatial data analyzer for the deutsch mapping codes using the dotenv method.
* */

fn main() {
    let argsparse = CommandParse::parse();
    match &argsparse.command {
        Commands::Plz { plz } => {
            let command = plz_mapper(plz).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Note { note } => {
            let command = note_mapper(note).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Einwohner { einwohner } => {
            let command = einwohner_mapper(einwohner).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Qkm { qkm } => {
            let command = qkm_mapper(qkm).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Latitude { lat } => {
            let command = latitude_mapper(lat).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Longitude { lon } => {
            let command = longitude_mapper(lon).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::OSM { osm_id } => {
            let command = osm_mapper(osm_id).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Ags { ags } => {
            let command = ags_mapper(ags).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Ord { ord } => {
            let command = ord_mapper(ord).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Landkreis { landkries } => {
            let command = landkries_mapper(landkries).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
        Commands::Bundesland { bundesland } => {
            let command = bundesland_mapper(bundesland).unwrap();
            println!(
                "The target from the corresponding hmm files have been filtered: {:?}",
                command
            );
        }
    }
}

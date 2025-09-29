use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "geomapper",
    version = "1.0",
    about = "geomapper, a geomapper for geman geographical data
   ************************************************
   Author Gaurav Sablok,
   Email: codeprog@icloud.com
   ************************************************ "
)]
pub struct CommandParse {
    /// subcommands for the specific actions
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// search according to the plz
    Plz { plz: String },
    /// search according to the note
    Note { note: String },
    /// search according to the einwohner
    Einwohner { einwohner: String },
    /// search according to the qkm
    Qkm { qkm: String },
    /// search according to the latitude
    Latitude { lat: String },
    /// search according to the longitude
    Longitude { lon: String },
    /// search according to the osm
    Osm { osm_id: String },
    /// search according to the ags
    Ags { ags: String },
    ///search according to the ort
    Ord { ord: String },
    /// search according to the landkries
    Landkreis { landkries: String },
    /// search according to the bundesland
    Bundesland { bundesland: String },
    /// general pattern search
    GeneralPattern { generalpattern: String },
}

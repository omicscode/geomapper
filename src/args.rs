use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = "deutsch-geomapper",
    version = "1.0",
    about = "deutsch geo-mapper"
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
    OSM { osm_id: String },
    /// search according to the ags
    Ags { ags: String },
    ///search according to the ort
    Ord { ord: String },
    /// search according to the landkries
    Landkreis { landkries: String },
    /// search according to the bundesland
    Bundesland { bundesland: String },
}

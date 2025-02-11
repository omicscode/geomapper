#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct MapperDE {
    pub plz: String,
    pub note: String,
    pub einwohner: String,
    pub qkm: String,
    pub lat: String,
    pub lon: String,
}

pub struct MapperOrt {
    pub osmid: String,
    pub ags: String,
    pub ord: String,
    pub plz: String,
    pub landries: String,
    pub bundesland: String,
}

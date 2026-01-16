
pub fn read() -> Vec<Vec<u8>> {
    let mut cr = csv::Reader::from_path("data/games.csv").expect("no file in data folder");
    let mut v: Vec<Vec<u8>> = Vec::new();
    for rec in cr.deserialize() {
        v.push(rec.expect("variable parse error"));
    }
    let hdr = v.pop().expect("no footer");
    for _ in 0..hdr[0] {
        v.push(Vec::new());
    }
    v
}

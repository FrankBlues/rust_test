use std::io::Cursor;
use bing_map::TileSystem;
use bing_map::TilesExtent;
// use reqwest::get;
use rand::Rng;

type Result1<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    let (lon0, lat0) =(116.177641, 39.924175);
    let (lon1, lat1) =(116.183095, 39.921244);
    let level = 18;
    let te = TilesExtent::new(lon0, lat0, lon1, lat1, level);
    let quad_keys = te.quad_keys();
    for q in &quad_keys {
        println!("{}", q);
    }
    
    // println!("{}", constuct_url(&quad_key, "a"));
    // let url = constuct_url(&quad_key, "a");

    // download_one_tile(&url);

}

/// Construct a bing map url.
/// a: Aerial Map  no labels; h: Aerial Map Style with labels
/// r: Road Map Style; ho: Old style
/// http://ecn.t{0-7}.tiles.virtualearth.net/tiles/{a, h, r, ho}132100103223330121.jpeg?g=129
/// http://h{0-3}.ortho.tiles.virtualearth.net/tiles/a132100103223330121.jpeg?g=129
fn constuct_url(quad_key: &str, style: &str) -> String {
    let domain = rand::thread_rng().gen_range(0..=7);
    format!("http://ecn.t{}.tiles.virtualearth.net/tiles/{}{}.jpeg?g=129", domain, style, quad_key)
}


fn download_one_tile(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url)?;
    let mut file = std::fs::File::create("d:/test.png")?;
    let mut content = Cursor::new(res.bytes()?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())

}

async fn fetch_url(url: String, file_name: String) -> Result1<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
pub mod download_utils {
    use std::io::Cursor;
    use std::path::PathBuf;
    // use reqwest::get;

    use rand::Rng;
    use futures::{stream, StreamExt};
    use reqwest::Client;
    const CONCURRENT_REQUESTS: usize = 8;

    /// Construct a bing map url.
    /// a: Aerial Map  no labels; h: Aerial Map Style with labels
    /// r: Road Map Style; ho: Old style
    /// http://ecn.t{0-7}.tiles.virtualearth.net/tiles/{a, h, r, ho}132100103223330121.jpeg?g=129
    /// http://h{0-3}.ortho.tiles.virtualearth.net/tiles/a132100103223330121.jpeg?g=129
    pub fn constuct_url(quad_key: &str, style: &str) -> String {
        let domain = rand::thread_rng().gen_range(0..=7);
        format!("http://ecn.t{}.tiles.virtualearth.net/tiles/{}{}.jpeg?g=129", domain, style, quad_key)
    }

    pub fn download_one_tile(url: &String, file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let res = reqwest::blocking::get(url)?;
        let mut file = std::fs::File::create(file_name)?;
        let mut content = Cursor::new(res.bytes()?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }
    
    pub async fn fetch_url(client: &reqwest::Client, url: &String, file_name: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let response = client.get(url).send().await?;
        let mut file = std::fs::File::create(file_name)?;
        let mut content =  Cursor::new(response.bytes().await?);
        std::io::copy(&mut content, &mut file)?;
        Ok(())
    }

    pub fn download_files(urls_files: Vec<(String, PathBuf)>) {
        for (u, f) in urls_files.iter() {
            println!("Downloading from {}", u);
            let result = download_one_tile(u, f);
            match result {
                Ok(()) => (),
                Err(error) => eprintln!("Problem downloading the file: {:?}", error),
            }
        }
    }

    pub async fn download_files_async(urls_files: Vec<(String, PathBuf)>) {
        let client = Client::new();
        let bodies = stream::iter(urls_files)
            .map(|(url, path)| {
                println!("Downloading from {}.", url);
                let client = &client;
                async move {
                    fetch_url(client, &url, &path).await
                }
            })
            .buffer_unordered(CONCURRENT_REQUESTS);

        // bodies.await;
        bodies
            .for_each(|b| async {
                // println!("Ok");
                match b {
                    Ok(()) => (),
                    Err(e) => eprintln!("Problem downloading the file: {}", e),
                }
            })
            .await;
    }

    /// parallel download use tokio::spawn
    pub async fn download_files_async1(urls_files: Vec<(String, PathBuf)>) {
        let client = Client::new();
        let bodies = stream::iter(urls_files)
            .map(|(url, path)| {
                println!("Downloading from {}.", url);
                let client = client.clone();
                tokio::spawn(async move {
                    fetch_url(&client, &url, &path).await
                })
            })
            .buffer_unordered(CONCURRENT_REQUESTS);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(b) => {
                        match b {
                            Ok(()) => (),
                            Err(e) => eprintln!("Problem downloading the file: {}", e),
                        }
                    }
                    Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
                }
            })
            .await;
    }
        

}
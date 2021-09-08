pub mod download_utils {
    use std::fs::read;
    use std::io::Cursor;
    use std::path::PathBuf;

    use futures::{stream, StreamExt};
    use rand::Rng;
    use reqwest::Client;
    const CONCURRENT_REQUESTS: usize = 8;

    /// Check whether or not download is needed
    /// Download only needed if local file exists and has the same length as the response
    /// from the server.  If any error happens during the process, download will need even
    /// though the condition satisfied.
    pub fn need_download(file_name: &PathBuf, url: &String) -> bool {
        if file_name.exists() {
            match reqwest::blocking::get(url) {
                Ok(res) => match res.bytes() {
                    Ok(res_bytes) => match read(file_name) {
                        Ok(file_bytes) => {
                            if file_bytes.len() == res_bytes.len() {
                                return false;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading local file {}", e);
                            return true;
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing the response {}", e);
                        return true;
                    }
                },
                Err(e) => {
                    eprintln!("Error request the url {}", e);
                    return true;
                }
            }
        }
        true
    }

    /// Check whether or not download is needed with blocking::Response
    /// Download only needed if local file exists and has the same length as the response
    /// from the server.  If any error happens during the process, download will need even
    /// though the condition satisfied.
    pub fn need_download_blocking(file_name: &PathBuf, url: &String) -> bool {
        if file_name.exists() {
            match reqwest::blocking::get(url) {
                Ok(res) => match res.bytes() {
                    Ok(res_bytes) => match read(file_name) {
                        Ok(file_bytes) => {
                            if file_bytes.len() == res_bytes.len() {
                                return false;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading local file {}", e);
                            return true;
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing the response {}", e);
                        return true;
                    }
                },
                Err(e) => {
                    eprintln!("Error request the url {}", e);
                    return true;
                }
            }
        }
        true
    }

    /// Check whether or not download is needed with Response
    /// Download only needed if local file exists and has the same length as the response
    /// from the server.  If any error happens during the process, download will need even
    /// though the condition satisfied.
    pub async fn need_download_async(file_name: &PathBuf, url: &String) -> bool {
        if file_name.exists() {
            match reqwest::get(url).await {
                Ok(res) => match res.bytes().await {
                    Ok(res_bytes) => match read(file_name) {
                        Ok(file_bytes) => {
                            if file_bytes.len() == res_bytes.len() {
                                return false;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading local file {}", e);
                            return true;
                        }
                    },
                    Err(e) => {
                        eprintln!("Error parsing the response {}", e);
                        return true;
                    }
                },
                Err(e) => {
                    eprintln!("Error request the url {}", e);
                    return true;
                }
            }
        }
        true
    }

    /// Construct a bing map url.
    /// a: Aerial Map  no labels; h: Aerial Map Style with labels
    /// r: Road Map Style; ho: Old style
    /// http://ecn.t{0-7}.tiles.virtualearth.net/tiles/{a, h, r, ho}132100103223330121.jpeg?g=129
    /// http://h{0-3}.ortho.tiles.virtualearth.net/tiles/a132100103223330121.jpeg?g=129
    pub fn constuct_url(quad_key: &str, style: &str) -> String {
        let domain = rand::thread_rng().gen_range(0..=7);
        format!(
            "http://ecn.t{}.tiles.virtualearth.net/tiles/{}{}.jpeg?g=129",
            domain, style, quad_key
        )
    }

    /// download one tile.
    pub fn download_one_tile(
        url: &String,
        file_name: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if need_download_blocking(file_name, url) {
            let res = reqwest::blocking::get(url)?;
            let mut file = std::fs::File::create(file_name)?;
            let mut content = Cursor::new(res.bytes()?);
            std::io::copy(&mut content, &mut file)?;
        }
        Ok(())
    }

    /// download one tile async.
    pub async fn fetch_url(
        client: &reqwest::Client,
        url: &String,
        file_name: &PathBuf,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if need_download_async(file_name, url).await {
            let response = client.get(url).send().await?;
            let mut file = std::fs::File::create(file_name)?;
            let mut content = Cursor::new(response.bytes().await?);
            std::io::copy(&mut content, &mut file)?;
        }
        Ok(())
    }

    /// download tiles one by one.
    pub fn download_files(urls_files: &Vec<(String, PathBuf)>) {
        for (u, f) in urls_files.iter() {
            // println!("Downloading from {}", u);
            let result = download_one_tile(u, f);
            match result {
                Ok(()) => (),
                Err(error) => eprintln!("Problem downloading the file: {:?}", error),
            }
        }
    }

    /// download tiles asyc.
    pub async fn download_files_async(urls_files: &Vec<(String, PathBuf)>) {
        let client = Client::new();
        let bodies = stream::iter(urls_files)
            .map(|(url, path)| {
                // println!("Downloading from {}.", url);
                let client = &client;
                async move { fetch_url(client, &url, &path).await }
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

    ///download cocurrently use tokio::spawn
    pub async fn download_files_async1(urls_files: &'static Vec<(String, PathBuf)>) {
        let client = Client::new();
        let bodies = stream::iter(urls_files)
            .map(|(url, path)| {
                println!("Downloading from {}.", url);
                let client = client.clone();
                tokio::spawn(async move { fetch_url(&client, &url, &path).await })
            })
            .buffer_unordered(CONCURRENT_REQUESTS);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(b) => match b {
                        Ok(()) => (),
                        Err(e) => eprintln!("Problem downloading the file: {}", e),
                    },
                    Err(e) => eprintln!("Got a tokio::JoinError: {}", e),
                }
            })
            .await;
    }
}

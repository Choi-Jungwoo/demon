use demon::Downloader;
use demon::Http;
use reqwest::blocking::Client;

#[test]
fn download() {
    let http = Http::from(("file url", Client::new()));
    let downloader = Downloader::from(http);
    downloader.save("save path", 8).unwrap();
}

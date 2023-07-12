mod http_downloader;

#[tokio::main]
async fn main() {
	let url = std::string::String::from("https://google.com/");
	println!("Using {} as a test download url", url);

	println!("Downloading with default-tls");
	let file_path = format!("default_tls_request_download");
	match http_downloader::http_download_file(&url, &file_path, core::time::Duration::from_secs(60 * 2), core::time::Duration::from_secs(60), 0, http_downloader::TlsBackend::Default){
		Ok(_) => {println!("default-tls is working");},
		Err(e) => {println!("default-tls is not working, {}", e);}
	}

	println!("Downloading with native-tls");
	let file_path = format!("native_tls_request_download");
	match http_downloader::http_download_file(&url, &file_path, core::time::Duration::from_secs(60 * 2), core::time::Duration::from_secs(60), 0, http_downloader::TlsBackend::NativeTls){
		Ok(_) => {println!("native-tls is working");},
		Err(e) => {println!("native-tls is not working, {}", e);}
	}

	println!("Downloading with rustls");
	let file_path = format!("rusttls_request_download");
	match http_downloader::http_download_file(&url, &file_path, core::time::Duration::from_secs(60 * 2), core::time::Duration::from_secs(60), 0, http_downloader::TlsBackend::Rustls){
		Ok(_) => {println!("rusttls is working");},
		Err(e) => {println!("rusttls is not working, {}", e);}
	}
}

use std::io::Write;
use std::io::Seek;

use futures::executor::block_on;

pub enum TlsBackend {
	Rustls,
	NativeTls,
	Default,
}

pub fn http_download<W: Write + Seek>(url: &String, writer: &mut W, timeout: core::time::Duration, connect_timeout: core::time::Duration, retries:usize, tls_backend: TlsBackend) -> Result<(), String>{
	let mut client_builder = reqwest::Client::builder()
			.timeout(timeout)
			.connect_timeout(connect_timeout);

	match tls_backend {
		TlsBackend::Rustls => {client_builder = client_builder.use_rustls_tls();},
		TlsBackend::NativeTls => {client_builder = client_builder.use_native_tls();},
		TlsBackend::Default => {},
	}

	let client = match client_builder.build(){
		Ok(c) => c,
		Err(e) => {
			return Err(format!("Failed creating reqwest client, {}", e));
		}
	};

	let mut tries:usize = 0;

	'retries_loop: loop{
		tries = tries + 1;
		let mut res = match block_on(client.get(url).send()){
			Ok(r) => r,
			Err(e) => {
				if tries <= retries{
					continue;
				}
				return Err(format!("HTTP get request to {} failed, {}", url, e));
			}
		};

		let status = res.status().as_u16();

		if status != 200 {
			if tries <= retries{
				continue;
			}
			return Err(format!("HTTP get request to {} responded with {}", url, status));
		}

		loop {
			match block_on(res.chunk()){
				Ok(chunk) => {
					match chunk {
						Some(bytes) => {
							writer.write_all(&bytes);
						},
						None => {
							break;
						}
					}
				},
				Err(e) => {
					if tries <= retries{
						continue 'retries_loop;
					}
					return Err(format!("Failed downloading body chunks, {}", e));
				},
			}
		}

		break;
	}
	return Ok(());
}

pub fn http_download_bytes(url: &String, timeout: core::time::Duration, connect_timeout: core::time::Duration, retries: usize, tls_backend: TlsBackend) -> Result<std::vec::Vec<u8>, String>{
	let mut buf = std::io::Cursor::new(std::vec::Vec::<u8>::new());
	match http_download(url, &mut buf, timeout, connect_timeout, retries, tls_backend){
		Ok(_) => {
			return Ok(buf.into_inner());
		},
		Err(e) => {
			return Err(e);
		},
	}
}

pub fn http_download_file(url: &String, file_path: &String, timeout: core::time::Duration, connect_timeout: core::time::Duration, retries: usize, tls_backend: TlsBackend) -> Result<(), String>{
	let mut file = match std::fs::OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(file_path){
		Ok(f) => f,
		Err(e) => {
			return Err(format!("Failed opening {} for writing, {}", file_path, e));
		}
	};

	match http_download(url, &mut file, timeout, connect_timeout, retries, tls_backend){
		Ok(_) => {
			return Ok(());
		},
		Err(e) => {
			return Err(e);
		},
	}	
}

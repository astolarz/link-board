use esp_idf_hal::io::Read;
use link_board::{data_retriever::DataRetriever, display::Route, env};
use embedded_svc::http::{client::Client, Method};
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

#[derive(Default)]
pub struct DataRetrieverImpl;

pub fn get_data_retriever() -> impl DataRetriever {
    DataRetrieverImpl::default()
}

impl DataRetrieverImpl {

}

impl DataRetriever for DataRetrieverImpl {
    async fn get_json_for_all_trains(&self) -> Result<Vec<(link_board::display::Route, String)>, link_board::error::Error> {
        // much of this code is from https://github.com/esp-rs/std-training/blob/main/intro/http-client/examples/https_client.rs
        let routes = vec![Route::Line1, Route::Line2];
        let urls = routes.into_iter().map(|route| (route, DataRetrieverImpl::url_for_route(route, env::api_key())));
        let mut results = Vec::with_capacity(urls.len());

        let connection = EspHttpConnection::new(&Configuration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            ..Default::default()
        }).unwrap();

        let mut client = Client::wrap(connection);
        let headers = [("accept", "text/plain")];

        for (current_route, current_url) in urls {
            let mut result_json = String::new();
            let request = client.request(Method::Get, &current_url.as_ref(), &headers).unwrap();
            let response = request.submit().unwrap();
            let status = response.status();

            log::info!("response code: {}", status);

            match status {
                200..=299 => {
                    let mut buf = [0_u8; 256];
                    let mut offset = 0;
                    let mut total = 0;
                    let mut reader = response;
                    loop {
                        if let Ok(size) = Read::read(&mut reader, &mut buf[offset..]) {
                            if size == 0 {
                                break;
                            }
                            total += size;
                            let size_plus_offset = size + offset;
                            match std::str::from_utf8(&buf[..size_plus_offset]) {
                                Ok(text) => {
                                    result_json.push_str(text);
                                    offset = 0;
                                }
                                Err(error) => {
                                    let valid_up_to = error.valid_up_to();
                                    buf.copy_within(valid_up_to.., 0);
                                    offset = size_plus_offset - valid_up_to;
                                }
                            }
                        }
                    }
                    println!("Total: {} bytes", total);
                    results.push((current_route, result_json));
                },
                _ => log::error!("unexpected response code: {}", status)
            }
        }

        Ok(results)
    }
}
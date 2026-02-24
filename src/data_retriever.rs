#![allow(async_fn_in_trait)]

use crate::{display::Route, error::Error};

const LINE_1_ROUTE_ID: &str = "40_100479";
const LINE_2_ROUTE_ID: &str = "40_2LINE";

pub trait DataRetriever {
    async fn get_json_for_all_trains(&self) -> Result<Vec<(Route, String)>, Error>;

    fn url_for_route(route: Route, api_key: String) -> String {
        let route_id = match route {
            Route::Line1 => LINE_1_ROUTE_ID,
            Route::Line2 => LINE_2_ROUTE_ID,
        };
        format!(
            "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/{}.json?includeSchedule=false&includeStatus=true&key={}",
            route_id,
            api_key
        )
    }
}

#[cfg(feature="cli")]
pub mod dr {
    use crate::{data_retriever::DataRetriever, display::Route, env, error::Error};

    const CONCURRENT_REQUESTS: usize = 2;
    
    pub fn get_data_retriever() -> impl DataRetriever {
        DataRetrieverImpl::default()
    }
    
    #[derive(Default)]
    pub struct DataRetrieverImpl;
    
    impl DataRetriever for DataRetrieverImpl {
        async fn get_json_for_all_trains(&self) -> Result<Vec<(Route, String)>, Error> {
            use futures::{stream, StreamExt};
            use log::debug;
    
            let routes = vec![Route::Line1, Route::Line2];
            let urls = routes.into_iter().map(|route| (route, DataRetrieverImpl::url_for_route(route, env::api_key())));
            let mut results = Vec::with_capacity(urls.len());
            let client =  reqwest::Client::new();
    
            let fetches = stream::iter(
                urls.map(|(route, url)| {
                    let mut results = vec![];
                    let client = client.clone();
                    async move {
                        match client.get(&url).send().await {
                            Ok(response) => {
                                match response.text().await {
                                    Ok(text) => {
                                        debug!("retrieved text of len {} for route {:?}", text.len(), route);
                                        results.push((route, text.to_owned()))
                                    },
                                    Err(e) => return Err(Error::client_error(e))
                                }
                            },
                            Err(e) => return Err(Error::client_error(e))
                        }
                        Ok(results)
                    }
                })
            ).buffer_unordered(CONCURRENT_REQUESTS).collect::<Vec<Result<Vec<(Route, String)>, Error>>>();
    
            for result in fetches.await {
                match result {
                    Ok(mut route_and_json) => results.append(&mut route_and_json),
                    Err(e) => return Err(e),
                }
            }
    
            debug!("retrieved {} results", results.len());
            Ok(results)
        }
    }
}

#[cfg(feature="esp32")]
pub mod dr {
    use esp_idf_hal::io::Read;
    use crate::{data_retriever::DataRetriever, display::Route, env};
    use embedded_svc::http::{client::Client, Method};
    use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

    #[derive(Default)]
    pub struct DataRetrieverImpl;

    pub fn get_data_retriever() -> DataRetrieverImpl {
        DataRetrieverImpl::default()
    }

    impl DataRetrieverImpl {
    }

    impl DataRetriever for DataRetrieverImpl {
        async fn get_json_for_all_trains(&self) -> Result<Vec<(link_board::display::Route, String)>, link_board::error::Error> {
            // much of this code is from https://github.com/esp-rs/std-training/blob/main/intro/http-client/examples/https_client.rs
            let routes = vec![Route::Line1, Route::Line2];
            log::info!("retrieving {} route(s)", routes.len());
            let urls = routes.into_iter().map(|route| (route, DataRetrieverImpl::url_for_route(route, env::api_key())));
            log::info!("got {} url(s)", urls.len());
            let mut results = Vec::with_capacity(urls.len());

            let connection = EspHttpConnection::new(&Configuration {
                use_global_ca_store: true,
                crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
                ..Default::default()
            }).unwrap();
            log::info!("got a connection");

            let mut client = Client::wrap(connection);
            log::info!("got a client");
            let headers = [("accept", "text/plain")];

            for (current_route, current_url) in urls {
                let mut result_json = String::new();
                let request = client.request(Method::Get, &current_url.as_ref(), &headers).unwrap();
                log::info!("submitting request...");
                let response = request.submit().unwrap();
                let status = response.status();

                log::info!("response code: {}", status);

                match status {
                    200..=299 => {
                        let mut buf = [0_u8; 2048];
                        let mut offset = 0;
                        let mut total = 0;
                        let mut reader = response;
                        loop {
                            if let Ok(size) = Read::read(&mut reader, &mut buf[offset..]) {
                                if size == 0 {
                                    break;
                                }
                                total += size;
                                log::info!("reading {} bytes, current total {} bytes", size, total);
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
                        log::info!("Total: {} bytes", total);
                        results.push((current_route, result_json));
                    },
                    _ => log::error!("unexpected response code: {}", status)
                }
            }

            Ok(results)
        }
    }
}
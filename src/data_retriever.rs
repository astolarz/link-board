#![allow(async_fn_in_trait)]

use crate::{display::Route, error::Error};

const LINE_1_ROUTE_ID: &str = "40_100479";
const LINE_2_ROUTE_ID: &str = "40_2LINE";

pub trait DataRetriever {
    async fn get_json_for_route(&mut self, route: Route) -> Result<String, Error>;
}

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

#[cfg(feature="cli")]
pub mod dr {
    use crate::{data_retriever::{DataRetriever, url_for_route}, display::Route, env, error::Error};
    
    pub fn get_data_retriever() -> impl DataRetriever {
        DataRetrieverImpl::new()
    }
    
    pub struct DataRetrieverImpl {
        client: reqwest::Client
    }

    impl DataRetrieverImpl {
        fn new() -> DataRetrieverImpl {
            Self {
                client: reqwest::Client::new()
            }
        }
    }
    
    impl DataRetriever for DataRetrieverImpl {
        async fn get_json_for_route(&mut self, route: Route) -> Result<String, Error> {
            let url = url_for_route(route, env::api_key());

            match self.client.get(&url).send().await {
                Ok(response) => {
                    match response.text().await {
                        Ok(text) => {
                            log::debug!("retrieved text of len {} for route {:?}", text.len(), route);
                            Ok(text)
                        },
                        Err(e) => return Err(Error::client_error(e))
                    }
                },
                Err(e) => return Err(Error::client_error(e))
            }
        }
    }
}

#[cfg(feature="esp32")]
pub mod dr {
    use crate::{data_retriever::{DataRetriever, url_for_route}, display::Route, env, error::Error};
    use esp_idf_hal::io::Read;
    use embedded_svc::http::{client::Client, Method};
    use esp_idf_svc::http::client::{Configuration, EspHttpConnection};

    pub struct DataRetrieverImpl {
        client: Client<EspHttpConnection>
    }

    pub fn get_data_retriever() -> DataRetrieverImpl {
        DataRetrieverImpl::new()
    }

    impl DataRetrieverImpl {
        fn new() -> DataRetrieverImpl {
            let connection = EspHttpConnection::new(&Configuration {
                use_global_ca_store: true,
                crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
                ..Default::default()
            }).unwrap();
            log::info!("got a connection");

            let client = Client::wrap(connection);
            log::info!("got a client");

            Self {
                client
            }
        }
    }

    impl DataRetriever for DataRetrieverImpl {
        async fn get_json_for_route(&mut self, route: Route) -> Result<String, Error> {
            // much of this code is from https://github.com/esp-rs/std-training/blob/main/intro/http-client/examples/https_client.rs

            let url = url_for_route(route, env::api_key());
            let headers = [("accept", "text/plain")];
            let mut result_json = String::new();

            let request = self.client.request(Method::Get, &url.as_ref(), &headers).unwrap();
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
                            log::debug!("reading {} bytes, current total {} bytes", size, total);
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
                },
                _ => log::error!("unexpected response code: {}", status)
            }

            Ok(result_json)
        }
    }
}
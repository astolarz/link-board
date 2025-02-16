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
            "https://api.pugetsound.onebusaway.org/api/where/trips-for-route/{}.json?includeSchedule=false&key={}",
            route_id,
            api_key
        )
    }
}

#[cfg(feature = "headless")]
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
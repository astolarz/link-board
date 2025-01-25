use link_board::data_retriever::DataRetriever;

#[derive(Default)]
pub struct DataRetrieverImpl;

pub fn get_data_retriever() -> impl DataRetriever {
    DataRetrieverImpl::default()
}

impl DataRetrieverImpl {

}

impl DataRetriever for DataRetrieverImpl {
    async fn get_json_for_all_trains(&self) -> Result<Vec<(link_board::display::Route, String)>, link_board::error::Error> {
        Ok(vec![])
    }
}
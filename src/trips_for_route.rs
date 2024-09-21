#[serde(rename_all="camelCase")]
pub struct TripsForRoute {
    code: u16,
    current_time: u64,
    data: Data,
    test: String,
    version: u16,
}

#[serde(rename_all="camelCase")]
struct Data {
    limit_exceeded: bool,
    list: Vec<Trip>,
    out_of_range: bool,
    references: References,
}

#[serde(rename_all="camelCase")]
struct Trip {

}

#[serde(rename_all="camelCase")]
struct Frequency {
    start_time: u64,
    end_time: u64,
    headway: u16,
}

struct References {

}
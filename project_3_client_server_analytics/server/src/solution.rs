use analytics_lib::{dataset::Dataset, query::Query, solution::compute_query_on_dataset};

// This is what runs on the SERVER when client calls hello()
pub fn hello() -> String {
    "hello".to_string()
}

// Step 1: slow_rpc - just return the dataset (clone it)
pub fn slow_rpc(dataset: &Dataset) -> Dataset {
    dataset.clone()
}

// Step 2: fast_rpc - run query on server, return only result
pub fn fast_rpc(dataset: &Dataset, query: &Query) -> Dataset {
    compute_query_on_dataset(dataset, query)
}
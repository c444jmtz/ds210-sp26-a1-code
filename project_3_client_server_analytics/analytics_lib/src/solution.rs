use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};
//Helper Function  
fn evaluate_condition(row: &Row, dataset: &Dataset, condition: &Condition) -> bool {
    match condition {
        Condition::Equal(col, val) => {
            let idx = dataset.column_index(col);
            row.get_value(idx) == val
        }
        Condition::Not(inner) => {
            !evaluate_condition(row, dataset, inner)
        }
        Condition::And(left, right) => {
            evaluate_condition(row, dataset, left) && evaluate_condition(row, dataset, right)
        }
        Condition::Or(left, right) => {
            evaluate_condition(row, dataset, left) || evaluate_condition(row, dataset, right)
        }
    }
}


pub fn filter_dataset(dataset: &Dataset, condition: &Condition) -> Dataset {
    let mut result = Dataset::new(dataset.columns().to_vec());
    for row in dataset.iter() {
        if evaluate_condition(row, dataset, condition) {
            result.add_row(row.clone());
        }
    }
    result
}

pub fn group_by_dataset(dataset: Dataset, column: &String) -> HashMap<Value, Dataset> { // Group the dataset by the specified column and return a HashMap where the key 
                                                                                        //  is the value of the group by column and the value is a Dataset containing all rows that belong to that group.
    let idx = dataset.column_index(column);
    let columns = dataset.columns().to_vec();
    let mut groups: HashMap<Value, Dataset> = HashMap::new();

    for row in dataset.into_iter() {
        let key = row.get_value(idx).clone();
        groups
            .entry(key)
            .or_insert_with(|| Dataset::new(columns.clone()))
            .add_row(row);
    }
    groups
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    
    return todo!()
}

pub fn compute_query_on_dataset(dataset: &Dataset, query: &Query) -> Dataset {
    let filtered = filter_dataset(dataset, query.get_filter());
    let grouped = group_by_dataset(filtered, query.get_group_by());
    let aggregated = aggregate_dataset(grouped, query.get_aggregate());

    // Create the name of the columns.
    let group_by_column_name = query.get_group_by();
    let group_by_column_type = dataset.column_type(group_by_column_name);
    let columns = vec![
        (group_by_column_name.clone(), group_by_column_type.clone()),
        (query.get_aggregate().get_result_column_name(), ColumnType::Integer),
    ];

    // Create result dataset object and fill it with the results.
    let mut result = Dataset::new(columns);
    for (grouped_value, aggregation_value) in aggregated {
        result.add_row(Row::new(vec![grouped_value, aggregation_value]));
    }
    return result;
}
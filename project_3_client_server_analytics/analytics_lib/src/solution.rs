use std::collections::HashMap;
use crate::dataset::{ColumnType, Dataset, Value, Row};
use crate::query::{Aggregation, Condition, Query};

pub fn filter_dataset(dataset: &Dataset, filter: &Condition) -> Dataset {
    todo!("Implement this!");
}

pub fn group_by_dataset(dataset: Dataset, group_by_column: &String) -> HashMap<Value, Dataset> {
    todo!("Implement this!");
}

//HELPER FUNCTIONS FOR aggregate_dataset

//helper to get the integer value 
fn get_int_value(row: &Row, col_index: usize) -> i32 {
    match row.get_value(col_index) { //.get_value from dataset.rs
        Value::Integer(value) => *value, //dereferences so we get actual integer value
        _ => panic!("Expected integer column"),
    }
}

//helper for sum of column
fn sum_column(dataset: &Dataset, column_name: &String) -> i32 {
    let col_index = dataset.column_index(column_name);
    let mut sum = 0;

    for row in dataset.iter() {
        sum += get_int_value(row, col_index); //simple accumulator
    }

    sum
}

//helper for avg of column
fn average_column(dataset: &Dataset, column_name: &String) -> i32 {
    let sum = sum_column(dataset, column_name);
    sum / dataset.len() as i32
}

pub fn aggregate_dataset(dataset: HashMap<Value, Dataset>, aggregation: &Aggregation) -> HashMap<Value, Value> {
    let mut result = HashMap::new();

    for (group_value, group_dataset) in dataset {
        let aggregated_value = match aggregation {
            Aggregation::Count(_column_name) => {
                Value::Integer(group_dataset.len() as i32)
            }

            Aggregation::Sum(column_name) => {
                Value::Integer(sum_column(&group_dataset, column_name)) //use helper function
            }

            Aggregation::Average(column_name) => {
                Value::Integer(average_column(&group_dataset, column_name)) //use helper function
            }
        };

        result.insert(group_value, aggregated_value);
    }

    result

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
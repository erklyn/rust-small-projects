struct FilterCondition {
    item: i32,
}

impl FilterCondition {
    fn is_match(&self, item: i32) -> bool {
        self.item == item
    }
}

fn custom_filter(collection: Vec<i32>, filter_condition: &FilterCondition) -> Vec<i32> {
    collection
        .into_iter()
        .filter(|item| filter_condition.is_match(*item))
        .collect()
}
fn main() {
    let filter_condition = FilterCondition { item: 42 };
    let vec_of_numbers = vec![44, 42, 84, 1337, 246, 21, 88, 42, 55, 54];
    let filtered_vec = custom_filter(vec_of_numbers, &filter_condition);
    println!("{:?}", filtered_vec);
}

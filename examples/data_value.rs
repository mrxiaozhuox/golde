fn main() {
    let t = serde_json::to_string(&vec![
        doson::DataValue::Number(1.0)
    ]);
    println!("{}", t.unwrap());
}
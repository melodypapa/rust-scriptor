use rust_scriptor::scriptor::OperationType;

#[test]
fn operation_type_to_string() {
    let op = OperationType::Add;
    assert_eq!(op.to_string(), "Add");

    let op = OperationType::Condition;
    assert_eq!(op.to_string(), "Condition");

    let op = OperationType::ForEach;
    assert_eq!(op.to_string(), "ForEach");

    let op = OperationType::Remove;
    assert_eq!(op.to_string(), "Remove");

    let op = OperationType::SetValue;
    assert_eq!(op.to_string(), "SetValue");

    let op = OperationType::SetEnabled;
    assert_eq!(op.to_string(), "SetEnabled");
}

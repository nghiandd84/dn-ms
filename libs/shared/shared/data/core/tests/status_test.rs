use shared_shared_data_core::status::*;

#[test]
fn created_holds_id() {
    let c = Created { id: 42 };
    assert_eq!(c.id, 42);
}

#[test]
fn deleted_wraps_value() {
    let d = Deleted("removed");
    assert_eq!(d.0, "removed");
}

#[test]
fn updated_wraps_value() {
    let u = Updated(123);
    assert_eq!(u.0, 123);
}

#[test]
fn ok_wraps_value() {
    let o = Ok(true);
    assert_eq!(o.0, true);
}

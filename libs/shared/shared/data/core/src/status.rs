pub struct Created {
    pub id: i32,
}

pub struct Deleted<T>(pub T);
pub struct Updated<T>(pub T);
pub struct Ok<T>(pub T);

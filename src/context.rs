use self::data::DataRepo;

pub mod data;

pub struct Context {
    pub data: DataRepo,
}

impl Context {
    pub fn new(data: DataRepo) -> Self {
        Self { data }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Context {
    fn default() -> Self {
        Self {
            data: DataRepo::default(),
        }
    }
}

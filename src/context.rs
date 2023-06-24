use self::data::DataRepo;


pub mod data;

pub struct Context {
    data: DataRepo,
}

impl Context {
    pub fn new(data: DataRepo) -> Self { Self { data } }
}

impl Default for Context {
    fn default() -> Self {
        Self { data: DataRepo::default() }
    }
}

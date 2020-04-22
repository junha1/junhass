pub trait CreateGlobal {
    type T: ?Sized;
    fn create() -> Box<T>;
}

pub trait CreateScript {
    type T: ?Sized;
    fn create(name: &str) -> Box<T>;
}



pub struct HandleFactory<T: ?Sized>;

pub struct ScriptSystem {
}
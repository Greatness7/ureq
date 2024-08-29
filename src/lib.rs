pub struct Request;

impl Request {
    pub fn call(self) -> Result<Response, ()> {
        unimplemented!()
    }
}

pub struct Response;

impl Response {
    pub fn into_reader(self) -> Box<dyn std::io::Read + Send + Sync + 'static> {
        unimplemented!()
    }
}

pub fn get(_: &str) -> Request {
    unimplemented!()
}

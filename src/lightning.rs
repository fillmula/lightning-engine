use std::{sync::Arc, collections::HashMap, net::SocketAddr, any::Any, convert::Infallible, future::Future};
use hyper::{Request, Response, Body, Server};
use hyper::service::{make_service_fn, service_fn};

type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub struct Req {
    request: Request<Body>,
    host: String,
    path: String,
    args: Vec<String>,
    query: String,
    hash: String
}

impl Req {

    fn new(request: Request<Body>) -> Req {
        return Req {
            request,
            args: Vec::new(),
            host: "".to_string(),
            path: "".to_string(),
            query: "".to_string(),
            hash: "".to_string(),
        }
    }

    pub fn get_host(&self) -> &String {
        return &self.host;
    }

    pub fn get_path(&self) -> &String {
        return &self.path;
    }

    pub fn get_args(&self) -> &Vec<String> {
        return &self.args;
    }

    pub fn get_hash(&self) -> &String {
        return &self.hash;
    }

    pub fn get_query(&self) -> &String {
        return &self.query;
    }
}

unsafe impl Sync for Req {}

unsafe impl Send for Req {}

pub struct Res {
    code: Option<u8>,
    headers: HashMap<&'static str, String>,
    body: String
}

impl Res {

    fn new() -> Res {
        Res {
            code: None,
            headers: HashMap::new(),
            body: "".to_string()
        }
    }
    pub fn set_code(&mut self, code: u8) {
        return self.code = Some(code);
    }

    pub fn get_code(&self) -> Option<u8> {
        return self.code;
    }

    pub fn set_body(&mut self, content: String) {
        self.body = content;
    }
}

unsafe impl Sync for Res {}

unsafe impl Send for Res {}


pub struct State {
    values: HashMap<String, Box<dyn Any>>
}

impl State {
    fn new() -> State {
        State {
            values: HashMap::new(),
        }
    }
}

unsafe impl Sync for State {}

unsafe impl Send for State {}

pub struct Ctx {
    req: Req,
    res: Res,
    state: State
}

impl Ctx {

    fn new(request: Request<Body>) -> Ctx {
        Ctx {
            req: Req::new(request),
            res: Res::new(),
            state: State::new()
        }
    }

    pub fn req(&self) -> &Req {
        return &self.req;
    }

    pub fn res(&mut self) -> &mut Res {
        return &mut self.res;
    }

    pub fn state(&mut self) -> &mut State {
        return &mut self.state;
    }
}

unsafe impl Sync for Ctx {}

unsafe impl Send for Ctx {}

pub type Next = dyn Fn() -> ();

pub type Middleware = dyn Fn(Ctx) -> Ctx + Send + Sync;

static mut MIDDLEWARES: Vec<Arc<Middleware>> = Vec::new();
const GETS: Vec<(&'static str, Arc<Middleware>)> = Vec::new();
const POSTS: Vec<(&'static str, Arc<Middleware>)> = Vec::new();
const PATCHES: Vec<(&'static str, Arc<Middleware>)> = Vec::new();
const DELETES: Vec<(&'static str, Arc<Middleware>)> = Vec::new();

pub fn get<F>(path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
    GETS.push((path, Arc::new(middleware)));
}

pub fn post<F>(path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
    POSTS.push((path, Arc::new(middleware)));
}

pub fn patch<F>(path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
    PATCHES.push((path, Arc::new(middleware)));
}

pub fn delete<F>(path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
    DELETES.push((path, Arc::new(middleware)));
}

pub fn install<F>(middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
    unsafe {
        MIDDLEWARES.push(Arc::new(middleware));
    }
}

fn apply(outer: Arc<Middleware>, inner: Arc<Middleware>) -> Arc<Middleware> {
    return Arc::new(move |mut ctx| {
        ctx = outer(ctx);
        return inner(ctx);
    });
}

fn build_middleware() -> Arc<Middleware> {
    unsafe {
        let length = MIDDLEWARES.len();
        println!("{}", length);
        if length == 0 { return Arc::new(|ctx| ctx) }
        if length == 1 {
            return MIDDLEWARES[0].clone();
         }
        let range = (0..(length - 1)).rev();
        let mut inner = MIDDLEWARES[length - 1].clone();
        for i in range {
            let outer = MIDDLEWARES[i].clone();
            inner = apply(outer, inner);
        }
        return inner;
    }
}

pub async fn listen(port: u16) {
    //let middleware = build_middleware().clone();
    println!("Start");
    let service = make_service_fn(move |_| {
        async {
            Ok::<_, GenericError>(service_fn(move |req| {
                println!("HERE RUN");
                async { handle_response(req, build_middleware()) }
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server = Server::bind(&addr).serve(service);
    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn handle_response(request: Request<Body>, middleware: Arc<Middleware>) -> Result<Response<Body>, GenericError> {
    let mut ctx = Ctx::new(request);
    ctx = middleware(ctx);
    Ok(Response::new(Body::from(ctx.res.body)))
}

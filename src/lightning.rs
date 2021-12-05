use std::{sync::Arc, collections::HashMap, net::SocketAddr, any::Any, convert::Infallible, future::Future};
use hyper::{Request, Response, Body, Server};
use hyper::service::{make_service_fn, service_fn};

type GenericError = Box<dyn std::error::Error + Send + Sync>;

pub struct Req {
    request: Request<Vec<u8>>,
    host: String,
    path: String,
    args: Vec<String>,
    query: String,
    hash: String
}

impl Req {
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
    pub fn set_code(&mut self, code: u8) {
        return self.code = Some(code);
    }

    pub fn get_code(&self) -> Option<u8> {
        return self.code;
    }
}

unsafe impl Sync for Res {}

unsafe impl Send for Res {}


pub struct State {
    values: HashMap<String, Box<dyn Any>>
}

unsafe impl Sync for State {}

unsafe impl Send for State {}

pub struct Ctx {
    req: Req,
    res: Res,
    state: State
}

impl Ctx {
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

pub struct App {
    middlewares: Vec<Arc<Middleware>>,
    gets: Vec<(&'static str, Box<Middleware>)>,
    posts: Vec<(&'static str, Box<Middleware>)>,
    patches: Vec<(&'static str, Box<Middleware>)>,
    deletes: Vec<(&'static str, Box<Middleware>)>,
}

impl App {

    pub fn new() -> App {
        return App {
            middlewares: Vec::new(),
            gets: Vec::new(),
            posts: Vec::new(),
            patches: Vec::new(),
            deletes: Vec::new(),
        };
    }

    pub fn get<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
        self.gets.push((path, Box::new(middleware)));
    }

    pub fn post<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
        self.posts.push((path, Box::new(middleware)));
    }

    pub fn patch<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
        self.patches.push((path, Box::new(middleware)));
    }

    pub fn delete<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
        self.deletes.push((path, Box::new(middleware)));
    }

    pub fn install<F>(&mut self, middleware: F) where F: 'static + Fn(Ctx) -> Ctx + Send + Sync {
        self.middlewares.push(Arc::new(middleware))
    }

    fn apply(&self, outer: Arc<Middleware>, inner: Arc<Middleware>) -> Arc<Middleware> {
        return Arc::new(move |mut ctx| {
            ctx = outer(ctx);
            return inner(ctx);
        });
    }

    fn build_middleware(&'static self) -> Arc<Middleware> {
        let length = self.middlewares.len();
        if length == 0 { return Arc::new(|ctx| ctx) }
        if length == 1 {
            return self.middlewares[0].clone();
         }
        let range = (0..(length - 1)).rev();
        let mut inner = self.middlewares[length - 1].clone();
        for i in range {
            let outer = self.middlewares[i].clone();
            inner = self.apply(outer, inner);
        }
        return inner;
    }

    pub async fn listen(&self, port: u16) {
        //let middleware = self.build_middleware();
        println!("Start");
        let service = make_service_fn(move |_| {
            async {
                Ok::<_, GenericError>(service_fn(move |req| {
                    async { response_examples(req) }
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
}

fn response_examples(req: Request<Body>) -> Result<Response<Body>, GenericError> {
    Ok(Response::new(Body::from("Hello from my code!")))
}

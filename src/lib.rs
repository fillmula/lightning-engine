pub mod lightning {

    use std::any::Any;
    use std::collections::HashMap;
    use hyper::{Request, Response, Body};
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

    pub struct State {
        values: HashMap<String, Box<dyn Any>>
    }

    pub type Next = dyn Fn() -> ();

    pub type Middleware<'a> = dyn Fn(&'a Req, &'a mut Res, &'a mut State, &'a Next) -> ();

    pub struct App {
        middlewares: Vec<&'static Middleware<'static>>,
        gets: Vec<(&'static str, &'static Middleware<'static>)>,
        posts: Vec<(&'static str, &'static Middleware<'static>)>,
        patches: Vec<(&'static str, &'static Middleware<'static>)>,
        deletes: Vec<(&'static str, &'static Middleware<'static>)>,
        applied_middleware: Option<Box<Middleware<'static>>>
    }

    impl App {

        pub fn new() -> App {
            return App {
                middlewares: Vec::new(),
                gets: Vec::new(),
                posts: Vec::new(),
                patches: Vec::new(),
                deletes: Vec::new(),
                applied_middleware: None
            };
        }

        pub fn get<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(&Req, &mut Res, &mut State, &Next) {
            self.gets.push((path, &middleware));
        }

        pub fn post<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(&Req, &mut Res, &mut State, &Next) {
            self.posts.push((path, &middleware));
        }

        pub fn patch<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(&Req, &mut Res, &mut State, &Next) {
            self.patches.push((path, &middleware));
        }

        pub fn delete<F>(&mut self, path: &'static str, middleware: F) where F: 'static + Fn(&Req, &mut Res, &mut State, &Next) {
            self.deletes.push((path, &middleware));
        }

        fn apply(&mut self, outer: &'static Middleware, inner: &'static Middleware) {
            self.applied_middleware = Some(Box::new(|req: &'static Req, res: &'static mut Res, state: &'static mut State, next: &'static Next| {
                outer(req, res, state, &|| {
                    //inner(req, res, state, next)
                });
            }));
            // self.applied_middleware = &|req: &Req, res: &mut Res, state: &mut State, next: &Next| {
            //     outer(req, res, state, &|| {
            //         inner(req, res, state, next)
            //     })
            // }

        }

        pub fn listen(&self, port: u16) {

        }
    }
}

fn my(req: &lightning::Req, res: &mut lightning::Res, state: &mut lightning::State, next: &lightning::Next) {

}

fn main() {

    let mut app = lightning::App::new();

    app.get("/abc", |req, res, state, next| {
        res.set_code(255);
    });

    app.get("/ddd", my);

    app.listen(3000);
}

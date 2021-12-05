pub mod lightning {

    use std::any::Any;
    use std::collections::HashMap;
    use hyper::{Request, Response, Body};

    struct Req {
        request: Request<Vec<u8>>,
        host: String,
        path: String,
        args: Vec<String>,
        query: String,
        hash: String
    }

    trait ReqMethods {
        fn get_host(&self) -> String;
        fn get_path(&self) -> String;
        fn get_args(&self) -> Vec<String>;
        fn get_query(&self) -> String;
        fn get_hash(&self) -> String;
    }

    impl ReqMethods for Req {

        fn get_host(&self) -> String {
            return self.host;
        }

        fn get_path(&self) -> String {
            return self.path;
        }

        fn get_args(&self) -> Vec<String> {
            return self.args;
        }

        fn get_hash(&self) -> String {
            return self.hash;
        }

        fn get_query(&self) -> String {
            return self.query;
        }
    }

    struct Res {
        code: u8,
        headers: HashMap<String, String>,
        body: String
    }

    struct State {
        values: HashMap<String, Box<dyn Any>>
    }

    type Next = dyn FnMut() -> ();

    type Handler = dyn FnMut(Req, Res, State, &Next) -> ();

    static MIDDLEWARES_VEC: Vec<Box<Handler>> = Vec::new();
    static GET_VEC: Vec<(String, Box<Handler>)> = Vec::new();
    static POST_VEC: Vec<(String, Box<Handler>)> = Vec::new();
    static PATCH_VEC: Vec<(String, Box<Handler>)> = Vec::new();
    static DELETE_VEC: Vec<(String, Box<Handler>)> = Vec::new();

    pub fn get<F: 'static +  Fn(Req, Res, State, &Next) -> ()>(path: String, handler: F) {
        GET_VEC.push((path, Box::new(handler)));
    }

    pub fn post<F: 'static +  Fn(Req, Res, State, &Next) -> ()>(path: String, handler: F) {
        POST_VEC.push((path, Box::new(handler)));
    }

    pub fn patch<F: 'static +  Fn(Req, Res, State, &Next) -> ()>(path: String, handler: F) {
        PATCH_VEC.push((path, Box::new(handler)));
    }

    pub fn delete<F: 'static +  Fn(Req, Res, State, &Next) -> ()>(path: String, handler: F) {
        DELETE_VEC.push((path, Box::new(handler)));
    }

    pub fn start(port: u16) {
        println!("start on port {}", port);
    }

    pub fn r#use<F: 'static + Fn(Req, Res, State, &Next) -> ()>(handler: F) {
        MIDDLEWARES_VEC.push(Box::new(handler));
    }

    // pub fn combine(handlers: Vec<&Handler>) -> &Handler {
    //     if handlers.len() == 1 { return handlers[0]; }
    //     let range = (0..(handlers.len() - 1)).rev();
    //     for i in range {

    //     }
    // }

    fn apply<F: 'static + Fn(Req, Res, State, &Next) -> (), G: 'static + Fn(Req, Res, State, &Next) -> ()>(outer: F, inner: G) -> impl 'static + Fn(Req, Res, State, &Next) -> () {
        return &|req, res, state, next| {
            return outer(req, res, state, &|| {
                return inner(req, res, state, next);
            });
        };
    }

    fn new_apply(
        outer: fn(Req, Res, State, &Next) -> (),
        inner: fn(Req, Res, State, &Next) -> ()) -> fn(Req, Res, State, &Next) -> () {
        return |req, res, state, next| {
            return outer(req, res, state, &|| {
                return inner(req, res, state, next);
            });
        }
    }

    fn try_try() {
        let a = 2;
        let outer = |req: Req, res: Res, state: State, next: &Next| {
            next();
        };
        let inner = |req: Req, res: Res, state: State, next: &Next| {
            next();
            print!("{}", a);
        };
        new_apply(outer, inner);

    }

    fn try_this() {
        get("/abc".to_string(), |req: Req, res: Res, state: State, next: &Next| {
            let path = req.get_path();

        })
    }
}

fn foo<F: Fn(i32) -> i32>(a: i32, f: F) -> impl Fn(i32) -> i32 {
    return &|a: i32| {
        a
     }
}

fn bar(a: fn(i16) -> i16) {

}

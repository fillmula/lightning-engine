#[path = "lightning.rs"] mod lightning;

#[tokio::main]
async fn main() {

    lightning::install(|ctx| {
        ctx.req().get_args();
        ctx.res().set_code(200);
        ctx.res().set_body("Hello from my!".to_string());
        println!("this middleware 1");
    });

    lightning::install(|ctx| {
        println!("this middleware 2");
    });

    lightning::get("/abc", |ctx| {
        ctx.res().set_code(200);
    });

    lightning::listen(5001).await;
}

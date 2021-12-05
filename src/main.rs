#[path = "lightning.rs"] mod lightning;

#[tokio::main]
async fn main() {

    lightning::install(|mut ctx| {
        println!("this middleware 1");
        ctx.res().set_body("abc".to_string());
        return ctx;
    });

    lightning::install(|ctx| {
        println!("this middleware 2");
        return ctx;
    });

    lightning::get("/abc", |mut ctx| {
        ctx.res().set_code(200);
        return ctx;
    });

    lightning::listen(5001).await;
}

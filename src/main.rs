#[path = "lightning.rs"] mod lightning;

#[tokio::main]
async fn main() {

    let mut app = lightning::App::new();

    app.install(|ctx| {
        println!("this middleware 1");
        return ctx;
    });

    app.install(|ctx| {
        println!("this middleware 2");
        return ctx;
    });

    app.get("/abc", |mut ctx| {
        ctx.res().set_code(200);
        return ctx;
    });

    app.listen(5001).await;
}

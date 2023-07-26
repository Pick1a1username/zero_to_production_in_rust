use zero_to_production_in_rust::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
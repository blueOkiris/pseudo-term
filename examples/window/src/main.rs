use pseudo_term::env::EnvironmentBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = EnvironmentBuilder::new("empty_room")
        .add_room("empty_room", &Vec::new())
        .build().await?;
    env.run().await?;
    Ok(())
}


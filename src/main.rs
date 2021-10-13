use aws_config::meta::region::RegionProviderChain;
use aws_sdk_apigateway::{Client, Error, Region, PKG_VERSION};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options{
    #[structopt(short, long)]
    region: Option<String>,

    #[structopt(short, long)]
    info: bool,
}

async fn get_all_apis(client: &Client) -> Result<(), Error>{
    let response =  client.get_rest_apis().send().await?;

    for rest_api in response.items.unwrap_or_default() {
        println!("ID:          {}", rest_api.id.as_deref().unwrap_or_default());
        println!("Name:        {}", rest_api.name.as_deref().unwrap_or_default());
        println!(
            "Description: {}",
            rest_api.description.as_deref().unwrap_or_default()
        );
        println!(
            "Version:     {}",
            rest_api.version.as_deref().unwrap_or_default()
        );
        println!("Created:     {}", rest_api.created_date.unwrap().to_chrono());
        println!();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let Options{
        region,
        info,
    } = Options::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-east-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);

    get_all_apis(&client).await;

    Ok(())
}

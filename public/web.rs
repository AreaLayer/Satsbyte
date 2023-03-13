// Import necessary libraries
use rocket::{get, post, routes, Rocket};
use rocket_contrib::templates::Template;
use serde::Serialize;

// Define the wallet structure
#[derive(Serialize)]
struct Wallet {
    balance: u64,
    lightning_channels: Vec<String>,
    fedimint_channels: Vec<String>,
}

// Define the main route to display the wallet
#[get("/")]
fn index() -> Template {
    // Create a new wallet instance
    let wallet = Wallet {
        balance: 1000,
        lightning_channels: vec!["channel1".to_string(), "channel2".to_string()],
        fedimint_channels: vec!["channel3".to_string(), "channel4".to_string()],
    };

    // Render the HTML template and pass the wallet data as context
    let context = json!({"wallet": wallet});
    Template::render("index", &context)
}

// Define the route to add funds to the wallet
#[post("/add_funds/<amount>")]
fn add_funds(amount: u64) -> Template {
    // Add the amount to the wallet balance
    // TODO: Implement actual logic to add funds to the wallet
    let wallet = Wallet {
        balance: 2000,
        lightning_channels: vec!["channel1".to_string(), "channel2".to_string()],
        fedimint_channels: vec!["channel3".to_string(), "channel4".to_string()],
    };

    // Render the HTML template and pass the updated wallet data as context
    let context = json!({"wallet": wallet});
    Template::render("index", &context)
}

// Define the routes for the web interface
fn routes() -> Vec<Route> {
    routes![
        index,
        add_funds
    ]
}

// Define the main function to start the web server
fn main() {
    // Create a new Rocket instance
    let rocket = rocket::ignite()
        .mount("/", routes())
        .attach(Template::fairing());

    // Launch the Rocket web server
    rocket.launch();
}

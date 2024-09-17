mod blockchain;
mod ai_integration;
mod privacy;
mod network;
mod supply_chain;
mod metaverse;
mod cashapp_integration;

#[tokio::main]
async fn main() {
    let mut blockchain = blockchain::N8IVChain::new();

    // Start AI integration tasks
    ai_integration::run_ai_tasks().await;

    // Start the blockchain node
    blockchain.start().await;

    // Example: Add a block with AI decision data
    blockchain.add_block(vec![blockchain.genesis_block.hash.clone()], "AI-analyzed Block".into());

    // Simulate supply chain integration
    supply_chain::track_item("Widget X", "Factory", "Retail");

    // CashApp integration
    let payment_confirmation = cashapp_integration::integrate_cashapp("#PurchaseWidgetX");
    println!("{}", payment_confirmation);

    println!("N8IV Blockchain started successfully.");
}

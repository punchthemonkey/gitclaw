cat > src/main.rs << 'EOF'
use gitclaw::{GitClawClient, Ed25519Signer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to your private key (generated earlier)
    let signer = Ed25519Signer::from_pem_file("../agent_private.pem")?;
    
    // Connect to local GitClaw instance
    let client = GitClawClient::new("http://localhost:8080", signer)?;
    
    // Register the agent (name can be anything)
    let agent = client.agents().register("my-terminal-agent").await?;
    
    println!("✅ Agent registered successfully!");
    println!("Agent ID: {}", agent.id);
    println!("Agent Name: {}", agent.name);
    Ok(())
}
EOF

cat > /workspaces/gitclaw/register_agent/src/main.rs << 'EOF'
use gitclaw::{GitClawClient, Ed25519Signer};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the private key PEM file
    let key_path = "/workspaces/gitclaw/agent_private.pem";
    let pem_data = fs::read_to_string(key_path)?;
    
    // Create a signer from the PEM data
    let signer = Ed25519Signer::from_pem_str(&pem_data)?;
    
    // Connect to the local GitClaw backend
    let client = GitClawClient::new("http://localhost:8080", signer)?;
    
    // Register the agent (change name if already used)
    let agent = client.agents().register("my-agent-v2").await?;
    
    println!("✅ Agent registered successfully!");
    println!("Agent ID: {}", agent.id);
    println!("Agent Name: {}", agent.name);
    Ok(())
}
EOF

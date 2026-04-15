# mtproxy
rust wrapper for getting mtproxy for telegram
# main
```rust
mod mtproxy;
use mtproxy::Mtproxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mtproxy_client = Mtproxy::new();

    match mtproxy_client.show_mtproxy_list().await {
        Ok(_) => {
        
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }
    Ok(())
}
   
```

# Launch (your script)
```
cargo run
```

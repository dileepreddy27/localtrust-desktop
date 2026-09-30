use localtrust_core::{Action, Broker};
use std::io;
fn main() -> Result<(), String> {
    let root = std::env::args().nth(1).unwrap_or_else(|| "demo-workspace".into());
    let mut broker = Broker::new(root)?;
    println!("LocalTrust offline CLI. Enter an action JSON (read/create). Ctrl+C exits.");
    loop {
        let mut line = String::new();
        if io::stdin().read_line(&mut line).map_err(|e| e.to_string())? == 0 { break; }
        let action: Action = match serde_json::from_str(&line) { Ok(a) => a, Err(e) => { println!("Rejected: {e}"); continue; } };
        let proposal = match broker.propose(action) { Ok(p) => p, Err(e) => { println!("Rejected: {e}"); continue; } };
        println!("{}\nType APPROVE to execute:", serde_json::to_string_pretty(&proposal).unwrap());
        line.clear(); io::stdin().read_line(&mut line).map_err(|e| e.to_string())?;
        println!("{:?}", broker.decide(proposal.id, line.trim() == "APPROVE"));
    }
    Ok(())
}

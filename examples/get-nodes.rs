use anyhow::Result;
use slurm_rs::Slurm;

#[tokio::main]
async fn main() -> Result<()> {
    let slurm = Slurm::new_from_env();
    println!("get slurm nodes");

    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_nodes().await?).unwrap()
    );

    println!("get a specific nodes info");
    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_node("teach-gpu-n0").await?).unwrap()
    );

    Ok(())
}

use anyhow::Result;
use slurm_rs::Slurm;

#[tokio::main]
async fn main() -> Result<()> {
    let slurm = Slurm::new_from_env();
    println!("get slurm diag");

    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_diag().await?).unwrap()
    );

    Ok(())
}

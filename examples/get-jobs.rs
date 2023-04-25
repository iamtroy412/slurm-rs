use anyhow::Result;
use slurm_rs::Slurm;

#[tokio::main]
async fn main() -> Result<()> {
    let slurm = Slurm::new_from_env();
    println!("get slurm jobs");

    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_jobs().await?).unwrap()
    );

    println!("get a specific jobs info");
    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_job("42").await?).unwrap()
    );

    Ok(())
}

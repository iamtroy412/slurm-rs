use anyhow::Result;
use slurm_rs::Slurm;

#[tokio::main]
async fn main() -> Result<()> {
    let slurm = Slurm::new_from_env();
    println!("get slurm partitions");

    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_partitions().await?).unwrap()
    );

    println!("get 'gpu' slurm partition");
    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_partition("gpu").await?).unwrap()
    );

    Ok(())
}

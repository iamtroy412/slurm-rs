use anyhow::Result;
use slurm_rs::Slurm;

#[tokio::main]
async fn main() -> Result<()> {
    let slurm = Slurm::new_from_env();
    println!("ping, slurm!");
    println!("{:?}", slurm.ping().await?);

    Ok(())
}

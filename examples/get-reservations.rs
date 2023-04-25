use anyhow::Result;
use slurm_rs::Slurm;

#[tokio::main]
async fn main() -> Result<()> {
    let slurm = Slurm::new_from_env();
    println!("get slurm reservations");

    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_reservations().await?).unwrap()
    );

    println!("get a specific reservations info");
    println!(
        "{}",
        serde_json::to_string_pretty(&slurm.get_reservation("res000").await?).unwrap()
    );

    Ok(())
}

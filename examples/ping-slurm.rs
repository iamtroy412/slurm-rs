use slurm_rs::Slurm;

fn main() {
    let _slurm = Slurm::new_from_env();
    println!("ping, slurm!");
}

use anyhow::Result;

const MIN_CORES_ALLOWED: u8 = 1;

pub fn nproc_command(all: bool, ignore: usize) -> Result<()> {
    let mut cores = if all {
        num_cpus::get_physical()
    } else {
        num_cpus::get()
    };

    // prevent overflow if --ignore N is greater than N cores
    if ignore >= cores {
        println!("{MIN_CORES_ALLOWED}");
        return Ok(());
    }

    if ignore != 0 {
        cores -= ignore
    }

    println!("{cores}");

    Ok(())
}

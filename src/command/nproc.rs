use anyhow::Result;

const MIN_CORES_ALLOWED: u8 = 1;

pub fn nproc_command(
    all: bool,
    ignore: usize,
    omp_num_limit: Option<usize>,
    omp_num_threads: Option<usize>,
) -> Result<()> {
    let mut cores = if all {
        num_cpus::get_physical()
    } else {
        num_cpus::get()
    };

    let sys_cores = cores;

    if !all {
        // OMP_NUM_LIMIT is applied first
        if let Some(limit) = omp_num_limit {
            if limit < sys_cores {
                cores = limit;
            }
        }

        // then OMP_NUM_THREADS should override everything
        if let Some(threads) = omp_num_threads {
            cores = threads;
        }
    }

    if ignore >= cores {
        // prevent underflow by returning minimum allowed
        println!("{MIN_CORES_ALLOWED}");
        return Ok(());
    }

    if ignore > 0 {
        cores -= ignore;
    }

    println!("{cores}");

    Ok(())
}

const MIN_CORES_ALLOWED: u8 = 1;

pub fn nproc_command(
    all: bool,
    ignore: usize,
    omp_num_limit: Option<usize>,
    omp_num_threads: Option<usize>,
) {
    let mut cores = if all {
        num_cpus::get_physical()
    } else if omp_num_threads.is_some() {
        // skip the check if OMP_NUM_THREADS is set
        omp_num_threads.expect("OMP_NUM_THREADS should be set")
    } else if let Some(limit) = omp_num_limit {
        // OMP_NUM_LIMIT is applied only if less than sys_cores
        let sys_cores = num_cpus::get();
        if limit < sys_cores {
            limit
        } else {
            sys_cores
        }
    } else {
        num_cpus::get()
    };

    if ignore >= cores {
        // prevent underflow by returning minimum allowed
        println!("{MIN_CORES_ALLOWED}");
        return;
    }

    cores -= ignore;

    println!("{cores}");
}

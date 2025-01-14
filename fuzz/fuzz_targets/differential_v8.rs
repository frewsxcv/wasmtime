#![no_main]

use libfuzzer_sys::arbitrary::{Result, Unstructured};
use libfuzzer_sys::fuzz_target;
use wasmtime_fuzzing::{generators, oracles};

fuzz_target!(|data: &[u8]| {
    // errors in `run` have to do with not enough input in `data`, which we
    // ignore here since it doesn't affect how we'd like to fuzz.
    drop(run(data));
});

fn run(data: &[u8]) -> Result<()> {
    let mut u = Unstructured::new(data);
    let mut config: generators::Config = u.arbitrary()?;
    config.set_differential_config();

    // Enable features that v8 has implemented
    config.module_config.config.simd_enabled = true;
    config.module_config.config.bulk_memory_enabled = true;
    config.module_config.config.reference_types_enabled = true;

    let module = config.generate(&mut u, Some(1000))?;
    oracles::differential_v8_execution(&module.to_bytes(), &config);
    Ok(())
}

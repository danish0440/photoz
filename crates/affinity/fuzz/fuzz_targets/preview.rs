#![no_main]

use libfuzzer_sys::fuzz_target;

// The whole native reader (archive, object stream, model) and the preview, on arbitrary bytes.
fuzz_target!(|data: &[u8]| {
    let _ = photoz_affinity::container::header(data);
    let _ = photoz_affinity::preview(data);
    let _ = photoz_affinity::read(data, photoz_affinity::Limits { max_entry: 16 << 20, max_total: 64 << 20 });
});

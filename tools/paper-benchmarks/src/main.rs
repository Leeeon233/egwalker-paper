#![allow(unused)]

use std::collections::HashMap;
use std::ops::Range;

use automerge::{AutoCommit, Automerge, ObjType, ReadDoc};
use automerge::transaction::Transactable;
use crdt_testdata::{load_testing_data, TestData, TestPatch};
#[cfg(feature = "bench")]
use criterion::{Bencher, BenchmarkId, black_box, Criterion, Throughput};
use diamond_types::AgentId;
use diamond_types::list::encoding::ENCODE_FULL;
use diamond_types::list::ListOpLog;
use diamond_types_crdt::list::ListCRDT as DTCRDT;
use jumprope::{JumpRope, JumpRopeBuf};
use serde::Serialize;
use trace_alloc::{get_thread_memory_usage, measure_memusage};
use yrs::{Text, TextRef, Transact};
#[cfg(feature = "bench")]
use crate::convert1::{bench_automerge_remote, convert_main};
#[cfg(feature = "bench")]
use crate::ff_bench::bench_ff;

mod benchmarks;
mod ff_bench;
mod convert1;

fn print_xf_sizes(name: &str, samples: usize) {
    let contents = std::fs::read(format!("benchmark_data/{name}.dt")).unwrap();
    println!("\n\nLoaded testing data from {} ({} bytes)", name, contents.len());

    let oplog = ListOpLog::load_from(&contents).unwrap();

    let sizes = oplog.get_size_stats_during_xf(samples, true);
    std::fs::write(format!("results/xf-{name}-ff.json"), serde_json::to_string(&sizes).unwrap());

    let sizes = oplog.get_size_stats_during_xf(samples, false);
    std::fs::write(format!("results/xf-{name}-noff.json"), serde_json::to_string(&sizes).unwrap());

    println!("Wrote xf-{name}-ff.json / xf-{name}-noff.json with {} entries", sizes.len());

    dbg!(oplog.get_ff_stats());
}

const DATASETS: &[&str] = &["S1", "S2", "S3", "C1", "C2", "A1", "A2"];

#[cfg(feature = "memusage")]
#[derive(Debug, Clone, Copy, Serialize)]
struct MemUsage {
    steady_state: usize,
    peak: usize,
}

// $ cargo run --features memusage --release
#[cfg(feature = "memusage")]
fn measure_memory() {
    let mut usage = HashMap::new();

    for &name in DATASETS {
        print!("{name}...");
        let filename = format!("../../datasets/{name}.am");
        let bytes = std::fs::read(&filename).unwrap();

        // The steady state is a jumprope object containing the document content.
        let (peak, steady_state, _) = measure_memusage(|| {
            let doc = AutoCommit::load(&bytes).unwrap();
            // black_box(doc);
            let (_, text_id) = doc.get(automerge::ROOT, "text").unwrap().unwrap();
            doc
        });

        println!(" peak memory: {peak} / steady state {steady_state}");
        usage.insert(name.to_string(), MemUsage { peak, steady_state });
    }

    let json_out = serde_json::to_vec_pretty(&usage).unwrap();
    let filename = "../../results/automerge_memusage.json";
    std::fs::write(filename, json_out).unwrap();
    println!("JSON written to {filename}");
}

fn main() {
//     // print_xf_sizes("friendsforever", 100);
//     // print_xf_sizes("clownschool", 100);
//     // print_xf_sizes("node_nodecc", 200);
//     // print_xf_sizes("git-makefile", 200);
//
//     // benchmarks::print_filesize();
//     // if cfg!(feature = "memusage") {
//     //     benchmarks::print_memusage();
//     // }

    #[cfg(feature = "memusage")]
    measure_memory();

    #[cfg(feature = "bench")] {
        let mut c = Criterion::default()
            .configure_from_args();

        bench_automerge_remote(&mut c);
        // bench_ff(&mut c);

        // benchmarks::local_benchmarks(&mut c);

        c.final_summary();

        // convert_main();
    }
}

pub fn linear_testing_data(name: &str) -> TestData {
    let filename = format!("../../editing-traces/sequential_traces/{}.json.gz", name);
    load_testing_data(&filename)
}

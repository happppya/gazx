// QuiZX - Rust library for quantum circuit rewriting and optimisation
//         using the ZX-calculus
// Copyright (C) 2021 - Aleks Kissinger
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONtS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// use std::time::Instant;
use quizx::circuit::*;
use quizx::extract::*;
use quizx::simplify::*;
use quizx::util;
use quizx::vec_graph::*;

use workspace::mutations::types::EdgeSpecified;
use std::mem::uninitialized;
use std::time::Duration;
use std::time::Instant;

// use quizx::tensor::*;
use workspace::mutations;
use workspace::mutations::mutation_runner;

static MEASURE_SUCCESS_RATE: bool = false;

fn run_mutation(
    circuit: &Circuit,
    mutation: &mutation_runner::MutationType
) -> Result<Duration, Box<dyn std::error::Error>> {
    let mut graph_experimental: Graph = circuit.to_graph();
    clifford_simp(&mut graph_experimental);

    //println!("control: {}", extracted_control.stats());

    let test_edge: Option<&EdgeSpecified> = None;
    let test_vertex = Some(10usize);
    let time_start = Instant::now();

    match mutation {
        mutation_runner::MutationType::LocalComplement =>
            mutations::local_complement(&mut graph_experimental, test_vertex),
        mutation_runner::MutationType::FullReduce =>
            mutations::full_reduce(&mut graph_experimental),
        mutation_runner::MutationType::Pivot =>
            mutations::pivot(&mut graph_experimental, test_edge),
        mutation_runner::MutationType::FlipEdge =>
            mutations::flip_edge(&mut graph_experimental, test_edge),
    }

    let duration = time_start.elapsed();

    //println!("used random edge: {:?}", test_edge);
    //println!("time elapsed: {:?}", duration);

    if MEASURE_SUCCESS_RATE {
        clifford_simp(&mut graph_experimental);
        graph_experimental.extractor().gflow().up_to_perm().extract()?;
    }

    //println!("experimental: {}", extracted_experimental.stats());

    Ok(duration)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const TRIALS: u32 = 100;

    let circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    for mutation in mutation_runner::MUTATIONS_EDGES {
        println!("\nRunning new mutation: {:?}", mutation);
        println!("Trials: {:?}", TRIALS);

        let mut total_nanoseconds: u128 = 0;
        let mut total_success: u32 = 0;
        let mut total_failure: u32 = 0;

        for _ in 0..TRIALS {
            let mutate_result = run_mutation(circuit, &mutation);
            match mutate_result {
                Ok(duration) => {
                    total_success += 1;
                    total_nanoseconds += duration.as_nanos();
                }
                Err(e) => {
                    total_failure += 1;
                }
            }
        }

        let average_nanoseconds = total_nanoseconds / (TRIALS as u128);
        let average_duration = Duration::from_nanos(average_nanoseconds as u64);
        let percent_success: f64 = ((total_success as f64) / (total_failure as f64)) * 100.0;

        println!("Average duration: {:?}", average_duration);
        
        if MEASURE_SUCCESS_RATE {
            println!(
                "Success rate: {:?} success / {:?} fail = {:?}%",
                total_success,
                total_failure,
                percent_success
            );
        }
    }

    Ok(())
}

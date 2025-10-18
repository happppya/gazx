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
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
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

fn run_mutation(
    circuit: &Circuit,
    mutation: &mutation_runner::MutationType,
) -> Result<Duration, Box<dyn std::error::Error>> {
    let mut graph_control: Graph = circuit.to_graph();
    let mut graph_experimental: Graph = circuit.to_graph();

    clifford_simp(&mut graph_control);
    clifford_simp(&mut graph_experimental);

    let extracted_control = &graph_control.extractor().gflow().up_to_perm().extract()?;

    println!("control: {}", extracted_control.stats());

    let test_edge: Option<&EdgeSpecified> = None;
    let test_vertex = Some(10usize);

    let time_start = Instant::now();
    match mutation {
        mutation_runner::MutationType::LocalComplement =>
            mutations::local_complement(&mut graph_experimental, test_vertex),
        mutation_runner::MutationType::FullReduce => mutations::full_reduce(&mut graph_experimental),
        mutation_runner::MutationType::Pivot =>
            mutations::pivot(&mut graph_experimental, test_edge),
        mutation_runner::MutationType::FlipEdge =>
            mutations::flip_edge(&mut graph_experimental, test_edge),
    }
    let duration = time_start.elapsed();

    println!("used random edge: {:?}", test_edge);
    println!("time elapsed: {:?}", duration);

    clifford_simp(&mut graph_experimental);
    let extracted_experimental = &graph_experimental.extractor().gflow().up_to_perm().extract()?;

    println!("experimental: {}", extracted_experimental.stats());

    Ok(duration)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    for mutation in mutation_runner::MUTATIONS_ALL {
        println!("\nRunning new mutation: {:?}", mutation);

        let duration = run_mutation(circuit, mutation);
        match duration {
            Ok(_) => println!("Mutation success"),
            Err(e) => println!("Mutation failed: {:?}", e),
        }
    }

    Ok(())
}

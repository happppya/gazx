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
use std::time::Instant;

// use quizx::tensor::*;
use workspace::mutations;

#[derive(Debug )]
enum MutationType {
    LocalComplement,
    FullReduce,
    Pivot,
}

fn verify_mutation(
    circuit: &Circuit,
    mutation: MutationType
) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph_control: Graph = circuit.to_graph();
    let mut graph_experimental: Graph = circuit.to_graph();

    clifford_simp(&mut graph_control);
    clifford_simp(&mut graph_experimental);

    let extracted_control = &graph_control.extractor().gflow().up_to_perm().extract()?;

    println!("control: {}", extracted_control.stats());

    match mutation {
        MutationType::LocalComplement =>
            mutations::local_complement(&mut graph_experimental, Some(50)),
        MutationType::FullReduce => mutations::full_reduce(&mut graph_experimental),
        MutationType::Pivot =>
            mutations::pivot(&mut graph_experimental, Some(&(1usize, 2usize, EType::N))),
    }

    clifford_simp(&mut graph_experimental);
    let extracted_experimental = &graph_experimental.extractor().gflow().up_to_perm().extract()?;

    println!("experimental: {}", extracted_experimental.stats());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let circuit = &Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();

    let mutations_to_run = vec![
        MutationType::LocalComplement,
        MutationType::FullReduce,
        MutationType::Pivot
    ];

    for mutation in mutations_to_run {
        println!("\nRunning new mutation: {:?}", mutation);

        let result = verify_mutation(circuit, mutation);
        match result {
            Ok(_) => println!("Mutation success"),
            Err(e) => println!("Mutation failed: {}", e),
        }
    }

    Ok(())
}

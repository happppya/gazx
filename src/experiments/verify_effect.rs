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
mod mutations;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let circuit = Circuit::from_file("circuits/small/grover_5.qasm")?.to_basic_gates();
  let mut graph_control: Graph = circuit.to_graph();
  let mut graph_experimental : Graph = circuit.to_graph();

  clifford_simp(&mut graph_control);
  clifford_simp(&mut graph_experimental);

  let extracted_control = &graph_control.extractor().gflow().up_to_perm().extract()?;
  
  println!("control: {}", extracted_control.stats());

  mutations::local_complement(&mut graph_experimental, Some(50));

  clifford_simp(&mut graph_experimental);
  let extracted_experimental = &graph_experimental.extractor().gflow().up_to_perm().extract()?;

  println!("experimental: {}", extracted_experimental.stats());

  Ok(())

}
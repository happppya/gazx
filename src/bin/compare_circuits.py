import numpy as np
import matplotlib.pyplot as plt

from qiskit import QuantumCircuit
from qiskit import transpile
from qiskit.qasm2 import dumps
from qiskit.quantum_info import Statevector, state_fidelity

import os

NAM_GATES = ['h', 'rz', 'cx', 'x']

def bhattacharyya_coefficient(p, q):
    return np.sum(np.sqrt(p * q))

def get_distribution(qc):
    qc = qc.remove_final_measurements(inplace=False)
    state = Statevector.from_instruction(qc)
    return state.probabilities()

def plot_distributions(p, q, title=""):
    n = len(p)
    x = np.arange(n)

    figure = plt.figure(figsize=(10, 5))
    plt.bar(x - 0.2, p, width=0.4, label="Goal")
    plt.bar(x + 0.2, q, width=0.4, label="Actual")
    
    plt.xticks(np.arange(n))

    plt.xlabel("Basis state index")
    plt.ylabel("Probability")
    plt.legend()
    plt.tight_layout()
    
    manager = figure.canvas.manager
    assert manager is not None, "Plot manager is None"
    
    manager.set_window_title(title)
    
    plt.show()

def get_circuit_stats(qc: QuantumCircuit):
    stats = {
        "total": 0,
        "1q": 0,
        "2q": 0,
        "nq": 0,
        "cx": 0, # CNOT gates
        "depth": qc.depth()
    }
    
    for instruction in qc.data:
        if instruction.operation.name not in ['barrier', 'measure']:
            stats["total"] += 1
            num_qubits = len(instruction.qubits)
            
            if num_qubits == 1: 
                stats["1q"] += 1
            elif num_qubits == 2: 
                stats["2q"] += 1
            else: 
                stats["nq"] += 1
            
            if instruction.operation.name == 'cx':
                stats["cx"] += 1
                
    return stats

def compare_circuits(expected_qc, circuit_list):

    p = get_distribution(expected_qc)
    goal_metrics = get_circuit_stats(expected_qc)
    
    results = []

    # Updated table formatting to fit the new CNOT column
    print(f"{'Circuit Variant':<20} | {'Total':<6} | {'2-Qubit':<8} | {'CNOTs':<6} | {'Depth':<6} | {'Bhattacharyya'}")
    print("-" * 84)
    print(f"{'GOAL':<20} | {goal_metrics['total']:<6} | {goal_metrics['2q']:<8} | {goal_metrics['cx']:<6} | {goal_metrics['depth']:<6} | 1.000000")

    for label, qc in circuit_list:
        stats = get_circuit_stats(qc)
        q = get_distribution(qc)
        bc = bhattacharyya_coefficient(p, q)
        
        print(f"{label:<20} | {stats['total']:<6} | {stats['2q']:<8} | {stats['cx']:<6} | {stats['depth']:<6} | {bc:.6f}")
        results.append((label, q))

    for label, q_dist in results:
        plot_distributions(p, q_dist, title=f"Goal vs {label}")

def transpile_and_save(qc, original_path):

    nam_qc : QuantumCircuit = transpile(qc, basis_gates=NAM_GATES, optimization_level=3)
    
    base_path, extension = os.path.splitext(original_path)
    new_path = f"{base_path}_namopt{extension}"
    
    with open(new_path, "w") as f:
        f.write(dumps(nam_qc))
    
    return nam_qc

if __name__ == "__main__":
    
    ACTUAL_PATH = "circuits/results/opt_mod5_4.qasm"
    
    expected_qc = QuantumCircuit.from_qasm_file("circuits/small/mod5_4.qasm")
    actual_qc = QuantumCircuit.from_qasm_file(ACTUAL_PATH)
    nam_qc = transpile_and_save(actual_qc, ACTUAL_PATH)

    circuits_to_compare = [
        ("Act Raw", actual_qc),
        ("Act Opt", nam_qc),
    ]

    compare_circuits(expected_qc, circuits_to_compare)
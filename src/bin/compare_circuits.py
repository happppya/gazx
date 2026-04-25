import numpy as np
import matplotlib.pyplot as plt

from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector, state_fidelity

def bhattacharyya_coefficient(p, q):
    return np.sum(np.sqrt(p * q))

def get_distribution(qc):
    qc = qc.remove_final_measurements(inplace=False)
    state = Statevector.from_instruction(qc)
    return state.probabilities()

def plot_distributions(p, q, title="Distribution Comparison"):
    n = len(p)
    x = np.arange(n)

    plt.figure(figsize=(10, 5))
    plt.bar(x - 0.2, p, width=0.4, label="Expected")
    plt.bar(x + 0.2, q, width=0.4, label="Actual")
    
    plt.xticks(np.arange(n))

    plt.xlabel("Basis state index")
    plt.ylabel("Probability")
    plt.title(title)
    plt.legend()
    plt.tight_layout()
    plt.show()
    
def compare_circuits(expected_qc, actual_qc):
    p = get_distribution(expected_qc)
    q = get_distribution(actual_qc)

    bc = bhattacharyya_coefficient(p, q)
    fidelity = state_fidelity(Statevector(p), Statevector(q))
    print(f"Bhattacharyya: {bc:.6f}")
    print(f"Fidelity: {fidelity:.6f}")

    plot_distributions(p, q)

if __name__ == "__main__":
    expected_qc = QuantumCircuit.from_qasm_file("circuits/small/mod5_4.qasm")
    actual_qc   = QuantumCircuit.from_qasm_file("circuits/results/mod5_4.qasm")

    compare_circuits(expected_qc, actual_qc)
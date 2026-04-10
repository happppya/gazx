import optuna
import subprocess
import sys
import os

RUST_EXEC = "target/release/genetic.exe" 

def objective(trial: optuna.Trial) -> float:
    
    elitism_rate = trial.suggest_float("elitism_rate", 0.0, 0.5)
    crossover_rate = trial.suggest_float("crossover_rate", 0.0, 0.5)
    tournament_size = trial.suggest_int("tournament_size", 2, 8)
    
    print(f"[Trial {trial.number}] starting with params: elitism_rate={elitism_rate:.4f}, crossover_rate={crossover_rate:.4f}, tournament_size={tournament_size}")
    
    cmd = [
        RUST_EXEC,
        "--elitism-rate", str(elitism_rate),
        "--crossover-rate", str(crossover_rate),
        "--tournament-size", str(tournament_size),
        "--population-size", "100", 
        "--generations", "128",      
        "--quiet"
    ]

    try:
        result = subprocess.run(
            cmd, 
            capture_output=True, 
            text=True, 
            check=True
        )
        
        output_str = result.stdout.strip().split('\n')[-1]
        score = float(output_str)
        
        print(f"[Trial {trial.number}] finished. Score: {score}")
        return score

    except subprocess.CalledProcessError as e:
        print(f"[Trial {trial.number}] failed with exit code {e.returncode}.")
        raise optuna.exceptions.TrialPruned()
    
if __name__ == "__main__":
    if not os.path.exists(RUST_EXEC):
        print(f"Could not find executable at {RUST_EXEC}")
        sys.exit(1)

    # local SQlite database to store results
    storage_name = "sqlite:///optuna_study.db"

    study = optuna.create_study(
        study_name="GA_Hyperparam_Optimization",
        direction="maximize",
        storage=storage_name,
        load_if_exists=True
    )
    
    # n_jobs=-1 uses all available CPU cores
    study.optimize(objective, n_trials=200, n_jobs=12)

    print("\nOptimization finish")
    print(f"Best Score: {study.best_value}")
    print("Best Hyperparameters:")
    for key, value in study.best_params.items():
        print(f"  {key}: {value}")
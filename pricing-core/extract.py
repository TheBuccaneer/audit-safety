import json
import glob
import os
import pandas as pd
import matplotlib.pyplot as plt

# Step 1: Ensure benchmarks have been run ('cargo bench' in pricing-core)
# Step 2: Extract data from Criterion output directory
rows = []
base = "target/criterion/BlackScholes_MC"
pattern = os.path.join(base, "*", "*", "new", "estimates.json")
for path in glob.glob(pattern):
    parts = path.split(os.sep)
    variant = parts[-4]      # e.g., 'seq' or 'par'
    paths = int(parts[-3])   # e.g., '100000'
    with open(path) as f:
        data = json.load(f)
    mean_ns = data["mean"]["point_estimate"]
    rows.append((variant, paths, mean_ns / 1e6))

# Create DataFrame and save CSV
df = pd.DataFrame(rows, columns=["variant","paths","mean_ms"])
df.to_csv("bench_results.csv", index=False)

# Pivot for heatmap
pivot = df.pivot(index="variant", columns="paths", values="mean_ms")

# Plot grouped bar chart
fig, ax = plt.subplots(figsize=(8, 5))
x = range(len(pivot.columns))
width = 0.35

ax.bar([i - width/2 for i in x], pivot.loc['seq'], width, label='seq')
ax.bar([i + width/2 for i in x], pivot.loc['par'], width, label='par')

ax.set_xticks(x)
ax.set_xticklabels([f"{p:,}" for p in pivot.columns])
ax.set_xlabel('Number of Paths')
ax.set_ylabel('Mean Runtime (ms)')
ax.set_title('Benchmark: Sequential vs. Parallel (Black-Scholes MC)')
ax.legend()
plt.tight_layout()
plt.savefig("figures/heatmap_bench.png", dpi=300)

if __name__ == "__main__":
    print("bench_results.csv and figures/heatmap_bench.png generated.")

import json
import glob
import os
import pandas as pd
import matplotlib.pyplot as plt

rows = []
base = "target/criterion/BlackScholes_MC"
# Pattern: target/criterion/BlackScholes_MC/{variant}/{paths}/new/estimates.json
pattern = os.path.join(base, "*", "*", "new", "estimates.json")
for path in glob.glob(pattern):
    # path z.B. "…/seq/100000/new/estimates.json"
    parts = path.split(os.sep)
    variant = parts[-4]      # seq or par
    paths = parts[-3]        # e.g. "100000"
    with open(path) as f:
        data = json.load(f)
    mean_ns = data["mean"]["point_estimate"]
    rows.append((variant, int(paths), mean_ns / 1e6))

df = pd.DataFrame(rows, columns=["variant","paths","mean_ms"])
# Speichern als CSV
df.to_csv("bench_results.csv", index=False)

# Pivot für Heatmap
pivot = df.pivot(index="variant", columns="paths", values="mean_ms")
plt.imshow(pivot, aspect="auto")
plt.colorbar(label="Laufzeit [ms]")
plt.xlabel("Paths")
plt.ylabel("Variante")
plt.xticks(range(len(pivot.columns)), pivot.columns)
plt.yticks(range(len(pivot.index)), pivot.index)
plt.title("BlackScholes MC: seq vs. par")
plt.tight_layout()
os.makedirs("figures", exist_ok=True)
plt.savefig("figures/heatmap_bench.png", dpi=300)

# Simple Rust N-BODY Simulation

Generate orbital data for the sun system.
```
cargo run --release > pos.out
```

Then plot the generated orbital data.
```
python3 -m venv python/venv
. python/venv/bin/activate
pip install -r python/requirements.txt
python python/plot.py
```
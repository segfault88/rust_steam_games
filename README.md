A toy Rust system used for manipulating a dataset of Steam games from Kaggle.

Highly unlikely that this is useful to anyone.

## Data

Data from: https://www.kaggle.com/datasets/fronkongames/steam-games-dataset unzip and place games.csv into data directory.

There is a mismatch in the number of columns (there is an extra one before `About the game`), so fix it up with the Python / pandas script:

```bash
uv run fixup.py
```

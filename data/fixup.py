# /// script
# requires-python = ">=3.12"
# dependencies = [
#     "pandas",
# ]
# ///
import pandas as pd

def replace_headers(input_file, output_file, corrected_headers):
    """
    Replaces the first line (headers) of a CSV file with corrected headers without reading the whole file.

    Args:
        input_file: Path to the input CSV file.
        output_file: Path to the output CSV file.
        corrected_headers: A list of corrected headers.
    """

    print("read csv")
    df = pd.read_csv(input_file, header=None, skiprows=1)
    print("set columns")
    df.columns = corrected_headers
    print("write csv")
    df.to_csv(output_file, index=False)


# Example usage
input_file = "games.csv"
output_file = "games-fixed.csv"
corrected_headers = [
    "AppID",
    "Name",
    "Release date",
    "Estimated owners",
    "Peak CCU",
    "Required age",
    "Price",
    "DiscountDLC count",
    "Mystery",
    "About the game",
    "Supported languages",
    "Full audio languages",
    "Reviews",
    "Header image",
    "Website",
    "Support url",
    "Support email",
    "Windows",
    "Mac",
    "Linux",
    "Metacritic score",
    "Metacritic url",
    "User score",
    "Positive",
    "Negative",
    "Score rank",
    "Achievements",
    "Recommendations",
    "Notes",
    "Average playtime forever",
    "Average playtime two weeks",
    "Median playtime forever",
    "Median playtime two weeks",
    "Developers",
    "Publishers",
    "Categories",
    "Genres",
    "Tags",
    "Screenshots",
    "Movies",
]

replace_headers(input_file, output_file, corrected_headers)

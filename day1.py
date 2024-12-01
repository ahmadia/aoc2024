import polars as pl

# Load the file treating any number of spaces as column separators
df = pl.read_csv(
    "1_input.txt",
    separator=" ",  # Treat single spaces as separators
    has_header=False
)

# Select only the columns of interest (ignore empty columns created by formatting
df = df.select([
    pl.col("column_1"),
    pl.col("column_4")
])

# Sort each column independently
sorted_col1 = df["column_1"].sort()
sorted_col2 = df["column_4"].sort()

# Calculate the absolute differences between the sorted columns
distances = (sorted_col1 - sorted_col2).abs()

# Sum the distances
total_distance = distances.sum()

# Display the result
print(f"Total Distance: {total_distance}")

# Now lets count occurences of the right column

import polars as pl

# Load the file treating any number of spaces as column separators
df = pl.read_csv(
    "1_input.txt",
    separator=" ",  # Treat single spaces as separators
    has_header=False
)

# Select only the columns of interest
df = df.select([
    pl.col("column_1").alias("left"),
    pl.col("column_4").alias("right")
])

# Count occurrences of each number on the right
right_counts = df.group_by("right").agg(
    count=pl.count("right")
)

# Filter the DataFrame to include only rows where "left" matches "right"
df_filtered = df.join(right_counts, left_on="left", right_on="right", how="inner")

# Calculate the score for matching rows
df_filtered = df_filtered.with_columns(
    (pl.col("left") * pl.col("count")).alias("score")
)

# Sum up the running score
total_score = df_filtered["score"].sum()

# Display the result
print(f"Total Score: {total_score}")
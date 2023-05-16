import pandas as pd

"""
This file parses the used words from the output.csv
which contains only the words that are actually used in original wordle game
"""

# read the CSV file
df = pd.read_csv('wordle.csv')

# filter the rows where the "day" column has a value
filtered_df = df[df['day'].notna()]

# get the words from the filtered dataframe
words = filtered_df['word'].tolist()

# write the words to a text file
with open('output.txt', 'w') as file:
    for word in words:
        file.write(word + '\n')

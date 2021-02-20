import pandas as pd 
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

## Import the data
df = pd.read_csv("plastics.csv")

## Minimal cleaning and selection of variables of interest.
int_cols = ["country", "grand_total"] 
nan = np.nan 

df["country"] = df.loc[:,"country"].str[:11].str.upper()
dfc = (df.query("grand_total != @nan")
       .loc[:,int_cols]
       .groupby(by="country")
       .sum()
       .sort_values("grand_total", ascending= False)
      .reset_index(level = 0))

## Plotting
plt.figure(figsize=(8,15))
fig = sns.barplot(x = "grand_total", y = "country", orient = "h", data = dfc)
fig.set_title("Total pollution by country (2019-2020)")
fig.set_ylabel("Country")
fig.set_xlabel("")
plt.xlim([1, 265000])

for index, value in enumerate(dfc.grand_total):
    plt.text(value + 1000, index + 0.25, str(int(value)), size = "small")

fig.get_figure().savefig("pollution_by_country.png", bbox_inches = "tight")
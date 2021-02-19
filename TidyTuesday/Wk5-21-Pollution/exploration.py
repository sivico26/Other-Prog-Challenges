import pandas as pd 
import numpy as np
import matplotlib.pyplot as plt
import seaborn as sns

## Import the data
df = pd.read_csv("plastics.csv")

## Minimal cleaning and selection of variables of interest.
int_cols = ["country", "grand_total"] 
nan = np.nan 

dfc = (df.query("year == 2019 & grand_total != @nan & country != 'EMPTY' ")
       .loc[:,int_cols]
       .groupby(by="country")
       .sum()
       .sort_values("grand_total")
      .reset_index(level = 0))

## Plotting
plt.figure(figsize=(8,15))
fig = sns.barplot(x = "grand_total", y = "country", orient = "h", data = dfc)
fig.set_xscale("log")
fig.set_title("Total pollution by country (2019)")
fig.set_ylabel("Country")
fig.set_xlabel("")
fig.get_figure().savefig("pollution_by_country.png", bbox_inches = "tight")
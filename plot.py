import matplotlib.pyplot as plt
import matplotlib.animation as animation
import matplotlib.patches as mpatches
import numpy as np 
import pandas as pd

data_frame = pd.read_csv("output.csv")
data_array = data_frame.to_numpy()
NUM_PARTICLES = len(data_array[0])
SKIP_MULTIPLIER = 1
X_MIN = -1.0
X_MAX = float(2* len(data_frame.columns))
DELTA_X = X_MAX - X_MIN
Y_MIN = -1.0
Y_MAX = 1.0

#Plotting tings
fig, ax = plt.subplots()

def update(frame):
    ax.cla()
    ax.set_xlim(X_MIN, X_MAX)
    ax.set_ylim(Y_MIN, Y_MAX)
    ax.set_xticks(range(int(X_MIN)+1, int(X_MAX), 2), labels=range(0, NUM_PARTICLES))
    ax.grid(visible=True, which='both', axis='both', color='b')
    ax.set_aspect(1/np.power(DELTA_X, 1/NUM_PARTICLES))
    for i, lat_point in enumerate(data_array[frame*SKIP_MULTIPLIER]):
        ax.add_patch(mpatches.FancyArrow(2*i, 0, np.cos(lat_point), np.sin(lat_point), width=.1, length_includes_head=True, color="C1"))

ani = animation.FuncAnimation(fig=fig, func=update, frames=int(np.ceil(len(data_array)/SKIP_MULTIPLIER)), interval = 1, )
plt.show()
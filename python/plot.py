import matplotlib.pyplot as plt

positions = []
with open('pos.out') as positions_file:
    for line in positions_file.readlines():
        i, x, y, z = map(float, line.split())
        positions.append((i, x, y, z))

unique = {i[0] for i in positions}
n = len(unique)

ii, xx, yy, zz = zip(*positions)

fig = plt.figure()
ax = fig.add_subplot(projection='3d')

view_radius = 2e11

for i in range(int(len(positions)/n)):
    plt.cla()
    ax.set(xlim=(-view_radius, view_radius), ylim=(-view_radius, view_radius))
    ax.scatter(xx[i:i+n],yy[i:i+n],zz[i:i+n],s=20)
    plt.pause(0.01)
#!/usr/bin/env python3

import matplotlib.pyplot as plt

if __name__ == '__main__':
    sites = {}
    with open(0) as fin:
        for line in fin:
            parts = line.split()
            i = parts[0]
            x = float(parts[1])
            y = float(parts[2])
            z = float(parts[3])
            neighbor_count = parts[4]
            neighbors = parts[5:]
            if len(neighbors) is not int(neighbor_count):
                raise Exception("Expecting {} neighbors; got {}".format(
                    neighbor_count, neighbors))
            sites[i] = {
                'x' : x, 'y' : y, 'z' : z,
                'neighbors' : neighbors
            }
            print(sites[i])

    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')
    for site_id in sites:
        site = sites[site_id]
        ax.scatter(site['x'], site['y'], site['z'])

    plt.show()

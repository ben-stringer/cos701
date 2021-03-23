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

            first_neighbor_count = int(parts[4])
            if first_neighbor_count == 0:
                first_neighbors = []
            else:
                first_neighbors = parts[5:first_neighbor_count+5]
            
            if not len(first_neighbors) == int(first_neighbor_count):
                raise Exception("Expecting {} first_neighbors; got {}".format(
                    first_neighbor_count, first_neighbors))

            second_neighbor_count = int(parts[first_neighbor_count+5])
            if second_neighbor_count == 0:
                second_neighbors = []
            else:
                second_neighbors = parts[first_neighbor_count+6:]

            if not len(second_neighbors) == int(second_neighbor_count):
                raise Exception("Expecting {} second_neighbors; got {}".format(
                    second_neighbor_count, second_neighbors))

            sites[i] = {
                'x' : x, 'y' : y, 'z' : z,
                'first_neighbors' : first_neighbors,
                'second_neighbors' : second_neighbors
            }

    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    x=[]
    y=[]
    z=[]
    for site_id in sites:
        site = sites[site_id]
        x.append(float(site['x']))
        y.append(float(site['y']))
        z.append(float(site['z']))
    ax.scatter(x,y,z,color='black')

    for site_id in sites:
        site = sites[site_id]
        for fn_id in site['first_neighbors']:
            fn = sites[fn_id]
            ax.plot(
                    [float(site['x']), float(fn['x'])],
                    [float(site['y']), float(fn['y'])],
                    [float(site['z']), float(fn['z'])],
                    color='blue')
        for sn_id in site['second_neighbors']:
            sn = sites[sn_id]
            ax.plot(
                    [float(site['x']), float(fn['x'])],
                    [float(site['y']), float(fn['y'])],
                    [float(site['z']), float(fn['z'])],
                    color='red')

    print("Showing plot")
    plt.show()

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
                first_neighbors = parts[5:first_neighbor_count + 5]

            if not len(first_neighbors) == int(first_neighbor_count):
                raise Exception("Expecting {} first_neighbors; got {}".format(
                    first_neighbor_count, first_neighbors))

            second_neighbor_count = int(parts[first_neighbor_count + 5])
            if second_neighbor_count == 0:
                second_neighbors = []
            else:
                second_neighbors = parts[first_neighbor_count + 6:]

            if not len(second_neighbors) == int(second_neighbor_count):
                raise Exception("Expecting {} second_neighbors; got {}".format(
                    second_neighbor_count, second_neighbors))

            sites[i] = {
                'x': x, 'y': y, 'z': z,
                'first_neighbors': first_neighbors,
                'second_neighbors': second_neighbors
            }

    fig = plt.figure()

    x = []
    y = []
    z = []

    site_250 = sites['250']
    site_250_sn = site_250['second_neighbors']
    site_250_fn = site_250['first_neighbors']
    for site_id in sites:
        if site_id == '250' or site_id in site_250_fn or site_id in site_250_sn:
            continue
        site = sites[site_id]
        x.append(float(site['x']))
        y.append(float(site['y']))
        z.append(float(site['z']))
    ax1 = fig.add_subplot(111, projection='3d')
    ax1.scatter(x, y, z, color='black', marker='.')

    ax1.scatter(float(site_250['x']), float(site_250['y']), float(site_250['z']), color='blue', marker='h')

    x.clear()
    y.clear()
    z.clear()
    for site_id in site_250_fn:
        site = sites[site_id]
        x.append(float(site['x']))
        y.append(float(site['y']))
        z.append(float(site['z']))
    ax1.scatter(x, y, z, color='red', marker='s')

    x.clear()
    y.clear()
    z.clear()
    for site_id in site_250_sn:
        site = sites[site_id]
        x.append(float(site['x']))
        y.append(float(site['y']))
        z.append(float(site['z']))
    ax1.scatter(x, y, z, color='green', marker='D')

    print("Showing plot")
    plt.show()

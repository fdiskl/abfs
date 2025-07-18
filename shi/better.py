#!/usr/bin/env python3

from astroquery.gaia import Gaia
import numpy as np

n = 1000

Gaia.MAIN_GAIA_TABLE = "gaiadr3.gaia_source"
Gaia.ROW_LIMIT = n

print("Querying Gaia archive...")
job = Gaia.launch_job_async(f"""
    SELECT TOP {n}
        source_id, ra, dec, parallax
    FROM gaiadr3.gaia_source
    WHERE parallax IS NOT NULL AND parallax > 0
    ORDER BY parallax DESC
""")
stars = job.get_results()
print("Downloaded", len(stars), "stars.")

# Convert to numpy arrays
parallax_mas = np.array(stars['parallax'])
ra_rad = np.deg2rad(np.array(stars['ra']))
dec_rad = np.deg2rad(np.array(stars['dec']))

# Convert to distance in light-years
distance_parsec = 1000.0 / parallax_mas
distance_ly = distance_parsec * 3.26156

# 3D Cartesian coordinates
x = distance_ly * np.cos(dec_rad) * np.cos(ra_rad)
y = distance_ly * np.cos(dec_rad) * np.sin(ra_rad)
z = distance_ly * np.sin(dec_rad)

coords = np.stack((x, y, z), axis=1)

# Open output file
print("Saving row-by-row to star_adjacency_matrix.txt ...")
with open(f"star_adjacency_matrix_{n}.txt", "w") as f:
    for i in range(coords.shape[0]):
        print(i/100, "%")
        dists_row = np.linalg.norm(coords[i] - coords, axis=1)
        row_str = " ".join(f"{d:.6f}" for d in dists_row)
        f.write(row_str + "\n")

print("Done.")

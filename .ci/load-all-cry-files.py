import glob
import cryptol

# Create a list of all `.cry` files.
cryptol_files = glob.glob(r"**/*.cry", recursive=True)

# Connect to `cryptol-remote-api`.
c = cryptol.connect()

# Attempt to load all `.cry` files. This will raise an exception if a
# file does not exist or load properly.
for f in cryptol_files:
    print(f'Loading {f}')
    c.load_file(f).result()

import os
from termcolors import *
print(color.RED + "Are you sure you want to clear cache?" + color.END)
answer = input("Your answer [Y/N]: ")
if answer == "Y" or answer == "y":
    os.system("rm -rf target && rm -rf __pycache__")
    print("Cache cleared.")
else:
    print("Aborting.")
    exit()

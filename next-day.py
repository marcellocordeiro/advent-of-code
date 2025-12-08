import os
import re
import sys

target_dir = sys.argv[1]

# Pattern: dayNN (NN = digits)
pattern = re.compile(r"day(\d+)$")

nums = []
for name in os.listdir(target_dir):
    m = pattern.match(name)
    if m:
        nums.append(int(m.group(1)))

next_num = (max(nums) + 1) if nums else 1
print(f"{next_num:02d}")

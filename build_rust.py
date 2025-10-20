#!/usr/bin/env python3
import subprocess
import sys
import os

# 获取参数
out_dir = sys.argv[1]

# 确保路径存在
os.makedirs(out_dir, exist_ok=True)

# 调用 cargo build
cmd = ["cargo", "build", "--release", "--target-dir", out_dir]
print("Running:", " ".join(cmd))
result = subprocess.run(cmd)

if result.returncode != 0:
    sys.exit(result.returncode)

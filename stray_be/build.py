import os
import shutil

os.system("cargo build")
if os.path.exists("python/api/stray_be.dll"):
    os.remove("python/api/stray_be.dll")
shutil.move("target/debug/stray_be.dll", "python/api")
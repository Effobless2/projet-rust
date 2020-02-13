from ctypes import cdll
lib = cdll.LoadLibrary("target/release/libbig_project.so")
x = lib.copy_ppm_file("./result.ppm", "./result2.ppm")
x = lib.generate_invert_ppm_file("./result.ppm", "./resultI.ppm")
x = lib.generate_grayscale_ppm_file("./result.ppm", "./resultG.ppm")
print("done!")
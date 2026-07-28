import os
import re
from collections import defaultdict

with open("crates/core/src/lib.rs", "r") as f:
    content = f.read()

# Create ffi directory
os.makedirs("crates/core/src/ffi", exist_ok=True)

# Extract macros and use statements
header = []
functions = []
in_func = False
curr_func = []
curr_name = ""

for line in content.split("\n"):
    if line.startswith("#![") or line.startswith("pub mod"):
        continue
    if line.startswith("#[no_mangle]"):
        in_func = True
        curr_func = [line]
    elif in_func:
        curr_func.append(line)
        if line.startswith("pub extern \"C\" fn "):
            match = re.search(r'fn ([a-zA-Z0-9_]+)\(', line)
            if match:
                curr_name = match.group(1)
        if line == "}":
            in_func = False
            functions.append((curr_name, "\n".join(curr_func)))
            curr_name = ""
            curr_func = []
    else:
        if line.strip() != "":
            header.append(line)

header_str = "\n".join(header)

groups = defaultdict(list)
for name, func_str in functions:
    if "gzip" in name:
        groups["gzip"].append(func_str)
    elif "markdown" in name:
        groups["markdown"].append(func_str)
    elif "fs_" in name:
        groups["fs"].append(func_str)
    elif "hash" in name or "verify_sha" in name:
        groups["hash"].append(func_str)
    elif "zip_" in name or "tar_" in name:
        groups["archive"].append(func_str)
    elif "http_" in name:
        groups["http"].append(func_str)
    elif "settings_" in name:
        groups["settings"].append(func_str)
    elif "parse_" in name:
        groups["mod_metadata"].append(func_str)
    elif "json_" in name:
        groups["json"].append(func_str)
    elif "natural_compare" in name or "human_readable" in name:
        groups["string"].append(func_str)
    elif "free" in name:
        groups["memory"].append(func_str)
    else:
        groups["misc"].append(func_str)

# Write out the modules
mod_stmts = []
for group, funcs in groups.items():
    filename = f"crates/core/src/ffi/{group}_ffi.rs"
    with open(filename, "w") as f:
        f.write("use std::ffi::{CStr, CString};\n")
        f.write("use std::os::raw::c_char;\n")
        f.write("use std::slice;\n")
        f.write("use crate::ffi::macros::*;\n\n")
        
        # some modules might need specific imports, let's just dump the whole header for safety in a macros.rs
        f.write("\n\n".join(funcs) + "\n")
    mod_stmts.append(f"pub mod {group}_ffi;")

# Create macros.rs
with open("crates/core/src/ffi/macros.rs", "w") as f:
    f.write(header_str + "\n")
mod_stmts.append("pub mod macros;")

# Create ffi/mod.rs
with open("crates/core/src/ffi/mod.rs", "w") as f:
    for stmt in sorted(mod_stmts):
        f.write(stmt + "\n")

# Rewrite lib.rs
with open("crates/core/src/lib.rs", "w") as f:
    f.write("#![allow(clippy::not_unsafe_ptr_arg_deref)]\n")
    f.write("pub mod cxx_bridge;\n")
    f.write("pub mod ffi;\n")

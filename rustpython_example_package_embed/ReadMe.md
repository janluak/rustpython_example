# Example from rustpython: package_embed
This is the example from rustpython with using a full python file within rust.
See original files [package_embed.rs](https://github.com/RustPython/RustPython/blob/main/examples/package_embed.rs) and [paclage_embed.py](https://github.com/RustPython/RustPython/blob/main/examples/package_embed.py).

## Issues
Not working out of the box.
`Cargo.toml` was modified for being able to compile and `vm.add_frozen(rustpython_pylib::frozen_stdlib());` as added for setting up the vm.

Still, python cannot run as a module isn't found.
```
File "/home/./package_embed.py", line 1, in <module>
    from dataclasses import dataclass
  File "dataclasses", line 5, in <module>
  File "inspect", line 35, in <module>
  File "dis", line 1, in <module>
ModuleNotFoundError: No module named '_dis'
```
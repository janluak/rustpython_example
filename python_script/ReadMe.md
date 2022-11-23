# main script from python
This is a new example for just having a basic python script without any imports or return values used in rust itself.

## Working
This example works after adding `vm.add_frozen(rustpython_pylib::frozen_stdlib());` to the creation of the vm.

Since the `main_module.py` is at the same directory as the `run` binary, no additional `python path` adding is required.

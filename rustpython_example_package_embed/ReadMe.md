# Example from rustpython: package_embed
This is the example from rustpython with using a full python file within rust.
See original files [package_embed.rs](https://github.com/RustPython/RustPython/blob/main/examples/package_embed.rs) and [paclage_embed.py](https://github.com/RustPython/RustPython/blob/main/examples/package_embed.py).

## Issues
Not working out of the box.
`Cargo.toml` was modified for being able to compile but at runtime main thread panics with:
```
Error in sys.excepthook:
lost sys.stderr
RuntimeError: lost sys.stderr
Original exception was:
lost sys.stderr
ImportError: No such frozen object named codecs
```
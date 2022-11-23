use rustpython_vm as vm;
use std::process::ExitCode;
use vm::{Interpreter};

fn py_main(interp: &Interpreter) -> vm::PyResult {
    interp.enter(|vm| {
        let module = vm.import("some_module", None, 0)?;
        let result = vm.invoke(&module, ())?;
        vm::PyResult::Ok(result)
    })
}

fn main() -> ExitCode {
    let interp = vm::Interpreter::with_init(Default::default(), |vm| {
        #[cfg(feature = "stdlib")]
        vm.add_native_modules(rustpython_stdlib::get_module_inits());

        #[cfg(feature = "freeze-stdlib")]
        vm.add_frozen(rustpython_pylib::frozen_stdlib());
    });
    let result = py_main(&interp);
    ExitCode::from(interp.run(|_vm| result))
}
use rustpython_vm as vm;
use std::process::ExitCode;
use vm::{Interpreter};

fn py_main(interp: &Interpreter) -> vm::PyResult {
    interp.enter(|vm| {
        let module = vm.import("some_module", None, 0)?;
        let name_func = module.get_attr("check_internet_access", vm)?;
        let result = vm.invoke(&name_func, ())?;
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
    // let result = result.map(|result| {   # ToDo how to print the raw string that is in the `PyObjectRef`
    //     println!("name: {}", result);
    // });
    ExitCode::from(interp.run(|_vm| result))
}
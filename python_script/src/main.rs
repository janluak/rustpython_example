use rustpython_vm as vm;
use std::process::ExitCode;
use vm::{Interpreter};

fn py_main(interp: &Interpreter) -> vm::PyResult {
    interp.enter(|vm| {
        vm.insert_sys_path(vm.new_pyobj("./"))
            .expect("add path");
        let module = vm.import("main_module", None, 0)?;
        let name_func = module.get_attr("main_func", vm)?;
        let result = vm.invoke(&name_func, ())?;
        vm::PyResult::Ok(result)
    })
}

fn main() -> ExitCode {
    let interp = vm::Interpreter::with_init(Default::default(), |vm| {
        vm.add_native_modules(rustpython_stdlib::get_module_inits());
    });
    let result = py_main(&interp);
    ExitCode::from(interp.run(|_vm| result))
}
use crate::{convert, vm_class::AccessibleVM, wasm_builtins::window};
use futures::{future, Future};
use js_sys::Promise;
use rustpython_vm::obj::objstr;
use rustpython_vm::pyobject::{PyContext, PyFuncArgs, PyObjectRef, PyResult, TypeProtocol};
use rustpython_vm::VirtualMachine;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};

enum FetchResponseFormat {
    Json,
    Text,
    ArrayBuffer,
}

impl FetchResponseFormat {
    fn from_str(vm: &mut VirtualMachine, s: &str) -> Result<Self, PyObjectRef> {
        match s {
            "json" => Ok(FetchResponseFormat::Json),
            "text" => Ok(FetchResponseFormat::Text),
            "array_buffer" => Ok(FetchResponseFormat::ArrayBuffer),
            _ => Err(vm.new_type_error("Unkown fetch response_format".into())),
        }
    }
    fn get_response(&self, response: &web_sys::Response) -> Result<Promise, JsValue> {
        match self {
            FetchResponseFormat::Json => response.json(),
            FetchResponseFormat::Text => response.text(),
            FetchResponseFormat::ArrayBuffer => response.array_buffer(),
        }
    }
}

fn browser_fetch(vm: &mut VirtualMachine, args: PyFuncArgs) -> PyResult {
    arg_check!(
        vm,
        args,
        required = [
            (url, Some(vm.ctx.str_type())),
            (handler, Some(vm.ctx.function_type()))
        ],
        optional = [(reject_handler, Some(vm.ctx.function_type()))]
    );
    let response_format =
        args.get_optional_kwarg_with_type("response_format", vm.ctx.str_type(), vm)?;
    let method = args.get_optional_kwarg_with_type("method", vm.ctx.str_type(), vm)?;
    let headers = args.get_optional_kwarg_with_type("headers", vm.ctx.dict_type(), vm)?;
    let body = args.get_optional_kwarg("body");
    let content_type = args.get_optional_kwarg_with_type("content_type", vm.ctx.str_type(), vm)?;

    let response_format = match response_format {
        Some(s) => FetchResponseFormat::from_str(vm, &objstr::get_value(&s))?,
        None => FetchResponseFormat::Text,
    };

    let mut opts = web_sys::RequestInit::new();

    match method {
        Some(s) => opts.method(&objstr::get_value(&s)),
        None => opts.method("GET"),
    };

    if let Some(body) = body {
        opts.body(Some(&convert::py_to_js(vm, body)));
    }

    let request = web_sys::Request::new_with_str_and_init(&objstr::get_value(url), &opts)
        .map_err(|err| convert::js_py_typeerror(vm, err))?;

    if let Some(headers) = headers {
        let h = request.headers();
        for (key, value) in rustpython_vm::obj::objdict::get_key_value_pairs(&headers) {
            let key = objstr::get_value(&vm.to_str(&key)?);
            let value = objstr::get_value(&vm.to_str(&value)?);
            h.set(&key, &value)
                .map_err(|err| convert::js_py_typeerror(vm, err))?;
        }
    }

    if let Some(content_type) = content_type {
        request
            .headers()
            .set("Content-Type", &objstr::get_value(&content_type))
            .map_err(|err| convert::js_py_typeerror(vm, err))?;
    }

    let window = window();
    let request_prom = window.fetch_with_request(&request);

    let handler = handler.clone();
    let reject_handler = reject_handler.cloned();

    let acc_vm = AccessibleVM::from_vm(vm);

    let future = JsFuture::from(request_prom)
        .and_then(move |val| {
            let response = val
                .dyn_into::<web_sys::Response>()
                .expect("val to be of type Response");
            response_format.get_response(&response)
        })
        .and_then(JsFuture::from)
        .then(move |val| {
            let vm = &mut acc_vm
                .upgrade()
                .expect("that the VM *not* be destroyed while promise is being resolved");
            match val {
                Ok(val) => {
                    let val = convert::js_to_py(vm, val);
                    let args = PyFuncArgs::new(vec![val], vec![]);
                    let _ = vm.invoke(handler, args);
                }
                Err(val) => {
                    if let Some(reject_handler) = reject_handler {
                        let val = convert::js_to_py(vm, val);
                        let args = PyFuncArgs::new(vec![val], vec![]);
                        let _ = vm.invoke(reject_handler, args);
                    }
                }
            }
            future::ok(JsValue::UNDEFINED)
        });
    future_to_promise(future);

    Ok(vm.get_none())
}

const BROWSER_NAME: &str = "browser";

pub fn mk_module(ctx: &PyContext) -> PyObjectRef {
    py_module!(ctx, BROWSER_NAME, {
        "fetch" => ctx.new_rustfunc(browser_fetch)
    })
}

pub fn setup_browser_module(vm: &mut VirtualMachine) {
    vm.stdlib_inits.insert(BROWSER_NAME.to_string(), mk_module);
}

use std::env;

// use wasmtime_wasi::WasiCtx;
use std::path::PathBuf;
use wasmtime::{component::*, Module, Table};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::bindings::{self};
use wasmtime_wasi::pipe::MemoryOutputPipe;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

mod wit {
    wasmtime::component::bindgen!({
        path:"../wit",
        world:"reactor",
    });
}

struct Host {
    table: ResourceTable,
    wasi: WasiCtx,
}

// impl wit::demo::utils::host::Host for Host {
//     fn greet(&mut self) -> wasmtime::Result<u8> {
//         Ok(10)
//     }
// }

impl WasiView for Host {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
        &mut self.wasi
    }
}

use wit::demo::utils::complex::Num;
use wit::demo::utils::http_types;
use wit::demo::utils::operation;
use wit::exports::demo::utils::inbound_http;

impl wit::demo::utils::complex::Host for Host {}
impl wit::demo::utils::http_types::Host for Host {}

impl operation::Host for Host {
    fn add(&mut self, a: Num, b: Num) -> wasmtime::Result<Num> {
        Ok(Num {
            real: a.real + b.real,
            imag: a.imag + b.imag,
        })
    }
}

// #[tokio::main]
fn main() -> wasmtime::Result<()> {
    // let engine = wasmtime::Engine::default();
    // let mut store = wasmtime::Store::new(&engine, ());
    // let bytes = std::fs::read("../target/wasm32-wasi/debug/guest.wasm")?;
    // let mut linker: wasmtime::component::Linker<WasiCtx> = wasmtime::component::Linker::new(&engine);
    env::set_var("WASMTIME_BACKTRACE_DETAILS", "1");
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(false);

    //In the context of wasmtime, Engine is a struct that represents the configuration and context for compiling WebAssembly modules. It is a central part of the Wasmtime API, responsible for managing the lifecycle of compiled WebAssembly code and providing the necessary environment for execution.

    let engine = Engine::new(&config)?;

    let component =
        Component::from_file(&engine, "guest.wasm").expect("guest.wasm not found");

    let mut linker = Linker::new(&engine);

    // Create our wasi context.
    // let mut builder = WasiCtxBuilder::new();

    // let stdout = MemoryOutputPipe::new(4096);
    // let stderr = MemoryOutputPipe::new(4096);
    // builder.stdout(stdout.clone()).stderr(stderr.clone());

    bindings::cli::environment::add_to_linker(&mut linker, |x| x)
        .expect("Unable to add environment");
    bindings::cli::exit::add_to_linker(&mut linker, |x| x).expect("Unable to add cli");
    bindings::io::error::add_to_linker(&mut linker, |x| x).expect("Unable to add io error");
    // bindings::sync::io::streams::add_to_linker(&mut linker, |x| x)
    //     .expect("Unable to add io streams");
    bindings::sync_io::io::streams::add_to_linker(&mut linker, |x| x)
        .expect("Unable to add io streams");
    bindings::cli::stdin::add_to_linker(&mut linker, |x| x).expect("Unable to add cli stdin");
    bindings::cli::stdout::add_to_linker(&mut linker, |x| x).expect("Unable to add cli stdout");
    bindings::cli::stderr::add_to_linker(&mut linker, |x| x).expect("Unable to add cli stderr");
    bindings::clocks::wall_clock::add_to_linker(&mut linker, |x| x)
        .expect("Unable to add clocks wallclock");
    // bindings::sync::filesystem::types::add_to_linker(&mut linker, |x| x)
    //     .expect("Unable to add filesystem types");
    bindings::sync_io::filesystem::types::add_to_linker(&mut linker, |x| x)
        .expect("Unable to add filesystem types");
    bindings::filesystem::preopens::add_to_linker(&mut linker, |x| x)
        .expect("Unable to add filesystem preopens");

    wit::Reactor::add_to_linker(&mut linker, |x| x)?;

    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, Host { table, wasi });

    let (reactor, instance) = wit::Reactor::instantiate(&mut store, &component, &linker)?;
    // let guest = reactor.demo_utils_guest();
    let guest = reactor.demo_utils_inbound_http();
    // let result = guest.call_get_data(&mut store)?;
    let request = inbound_http::Request {
        method: http_types::Method::Get,
        uri: "http://google.com".to_string(),
        headers: vec![(String::from("method"), String::from("get"))],
        params: vec![],
        body: None,
    };
    let result = guest.call_handle_request(&mut store, &request);
    match result {
        Ok(res) => println!("Response : {:?}", res),
        Err(err) => println!("Error occured : {:?}", err),
    };
    // let linker = wasmtime::Linker::new(&engine);
    // let module_instance = linker
    //     .instantiate(&mut store, &module)
    //     .expect("Unable to get module instance");
    // let bye = module_instance.get_typed_func::<(), u32>(&mut store, "bye")?;

    // let bye_result = bye.call(&mut store, ()).expect("Function run failed");
    // println!("From bye {}",bye_result);

    // let callme = instance.get_typed_func::<(), ()>(&mut store, "getData")?;
    // callme.call(&mut store, ())?;
    Ok(())
}

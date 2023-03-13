use deno_core::futures::executor;
use deno_core::Extension;
use rust_embed::RustEmbed;

// #[derive(RustEmbed)]
// #[folder = "src/deno/"]
// #[include = "runtime.js"]
// pub struct Runtime;

use deno_core::error::AnyError;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use crate::config;

use super::ext;

fn get_error_class_name(e: &AnyError) -> &'static str {
	deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub fn start_macro(macro_name: &str) {
	let macro_path = config::get_config_directory()
		.join(macro_name)
		.join("index.js");
	println!("{:?}", macro_path);

	let module_loader = Rc::new(FsModuleLoader);
	let create_web_worker_cb = Arc::new(|_| {
		todo!("Web workers are not supported in the example");
	});
	let web_worker_event_cb = Arc::new(|_| {
		todo!("Web workers are not supported in the example");
	});

	let options = WorkerOptions {
		bootstrap: BootstrapOptions {
			args: vec![],
			cpu_count: 1,
			debug_flag: false,
			enable_testing_features: false,
			locale: deno_core::v8::icu::get_language_tag(),
			location: None,
			no_color: false,
			is_tty: false,
			runtime_version: "x".to_string(),
			ts_version: "x".to_string(),
			unstable: false,
			user_agent: "scrype".to_string(),
			inspect: false,
		},
		extensions: vec![ext::init_ops()],
		startup_snapshot: None,
		unsafely_ignore_certificate_errors: None,
		root_cert_store: None,
		seed: None,
		source_map_getter: None,
		format_js_error_fn: None,
		web_worker_preload_module_cb: web_worker_event_cb.clone(),
		web_worker_pre_execute_module_cb: web_worker_event_cb,
		create_web_worker_cb,
		maybe_inspector_server: None,
		should_break_on_first_statement: false,
		should_wait_for_inspector_session: false,
		module_loader,
		npm_resolver: None,
		get_error_class_fn: Some(&get_error_class_name),
		cache_storage_dir: None,
		origin_storage_dir: None,
		blob_store: BlobStore::default(),
		broadcast_channel: InMemoryBroadcastChannel::default(),
		shared_array_buffer_store: None,
		compiled_wasm_module_store: None,
		stdio: Default::default(),
		leak_isolate: true,
	};

	let main_module =
		deno_core::resolve_path(&macro_path.to_string_lossy()).unwrap();
	let permissions = PermissionsContainer::allow_all();

	let mut worker = MainWorker::bootstrap_from_options(
		main_module.clone(),
		permissions,
		options,
	);
	let worker_result =
		executor::block_on(worker.execute_main_module(&main_module));
	if let Err(err) = worker_result {
		println!("{:?}", err);
	}

	let worker_result = executor::block_on(worker.run_event_loop(false));
	if let Err(err) = worker_result {
		println!("{:?}", err);
	}
}

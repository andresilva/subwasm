use super::{
	calls::{call::Call, error::Error, event::Event, prelude::PalletId},
	pallet_data::PalletData,
	pallet_item::PalletItem,
	reduced_pallet::ReducedPallet,
};
use crate::differs::reduced::calls::{
	call::variant_to_calls, constant::Constant, error::variant_to_errors, event::variant_to_events, storage::*,
};
use comparable::Comparable;
use frame_metadata::{
	v14, PalletCallMetadata, PalletMetadata,
	RuntimeMetadata::{self, *},
};
use scale_info::{form::PortableForm, PortableRegistry};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

pub type ReducedRuntimeError = String;
pub type Result<T> = core::result::Result<T, ReducedRuntimeError>;

// TODO: fix the variables names in here

#[derive(Debug, PartialEq, Comparable, Serialize)]
pub struct ReducedRuntime {
	pub pallets: HashMap<PalletId, ReducedPallet>,
}

// TODO: One of the following is wrong
impl From<&PalletCallMetadata<PortableForm>> for PalletItem {
	fn from(fn_meta: &PalletCallMetadata<PortableForm>) -> Self {
		PalletItem::Call(fn_meta.into())
	}
}

impl From<&PalletCallMetadata<PortableForm>> for PalletData {
	fn from(call: &PalletCallMetadata<PortableForm>) -> Self {
		Self { name: "todo".to_string(), index: None, signature: Box::new(call.ty), docs: vec![] }
	}
}

impl From<HashMap<PalletId, ReducedPallet>> for ReducedRuntime {
	fn from(pallets: HashMap<PalletId, ReducedPallet>) -> Self {
		Self { pallets }
	}
}

// TODO: impl Iterator / IntoIterator
impl ReducedRuntime {
	#[cfg(feature = "v13")]
	/// Reduce a RuntimeMetadataV13 into a normalized ReducedRuntime
	pub fn from_v13(v13: &v13::RuntimeMetadataV13) -> Result<Self> {
		let mut pallets = convert(&v13.modules).clone();
		// TODO: we may not need to sort
		pallets.sort_by(|a, b| a.index.cmp(&b.index));

		let reduced_pallets: Vec<ReducedPallet> = pallets.iter().map(|p| p.into()).collect();
		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}

	pub fn get_reduced_pallet_from_v14_pallet(
		p: &PalletMetadata<PortableForm>,
		registry: &PortableRegistry,
	) -> ReducedPallet {
		let name = &p.name;

		// calls
		let calls = if let Some(calls) = &p.calls {
			let id = calls.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let calls: BTreeMap<PalletId, Call> = variant_to_calls(v);

					// calls.iter().for_each(|call| println!("  call = {}", call));
					calls
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no calls", &p.name);
			BTreeMap::new()
		};

		// events
		let events = if let Some(item) = &p.event {
			let id = item.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let events: BTreeMap<PalletId, Event> = variant_to_events(v);

					// events.iter().for_each(|event| println!("  event = {}", event));
					events
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no events", &p.name);
			BTreeMap::new()
		};

		// errors
		let errors = if let Some(item) = &p.error {
			let id = item.ty.id();
			let ty = registry.resolve(id.to_owned()).unwrap();

			match ty.type_def() {
				scale_info::TypeDef::Variant(v) => {
					let errors: BTreeMap<PalletId, Error> = variant_to_errors(v);
					errors
				}
				_ => unimplemented!(),
			}
		} else {
			// println!("   {} has no errors", &p.name);
			BTreeMap::new()
		};

		// storages
		let storages = if let Some(item) = &p.storage {
			item.entries
				.iter()
				.map(|e| {
					(
						e.name.clone(),
						Storage { name: e.name.clone(), docs: e.docs.clone(), default_value: e.default.clone() },
					)
				})
				.collect()
		} else {
			// println!("   {} has no storage", &p.name);
			BTreeMap::new()
		};

		// constants
		let constants: BTreeMap<String, Constant> = p
			.constants
			.iter()
			.map(|i| (i.name.clone(), Constant::new(&i.name, i.value.clone(), i.docs.clone())))
			.collect();

		ReducedPallet { index: p.index.into(), name: name.into(), calls, events, errors, constants, storages }
	}

	#[cfg(feature = "v14")]
	/// Reduce a RuntimeMetadataV14 into a normalized ReducedRuntime
	pub fn from_v14(v14: &v14::RuntimeMetadataV14) -> Result<Self> {
		let registry = &v14.types;

		// TODO: deal with extrinsic as well
		let _extrinsics = &v14.extrinsic;

		let pallets = &v14.pallets;
		let reduced_pallets: HashMap<PalletId, ReducedPallet> = pallets
			.iter()
			.map(|p| {
				let reduced_runtime = ReducedRuntime::get_reduced_pallet_from_v14_pallet(p, registry);
				(reduced_runtime.index, reduced_runtime)
			})
			.collect();

		let r_rtm: ReducedRuntime = reduced_pallets.into();
		Ok(r_rtm)
	}

	/// Prefer using the more efficient [get_pallet_by_id] if you can.
	pub fn get_pallet_by_name(&self, pallet_name: &str) -> Option<&ReducedPallet> {
		self.pallets.values().find(|reduced_pallet| reduced_pallet.name == pallet_name)
	}

	/// You can also use [get_pallet_by_name] but prefer using [get_pallet_by_id] where you can.
	pub fn get_pallet_by_id(&self, pallet_id: PalletId) -> Option<&ReducedPallet> {
		self.pallets.get(&pallet_id)
	}
}

impl From<&RuntimeMetadata> for ReducedRuntime {
	fn from(runtime_metadata: &RuntimeMetadata) -> Self {
		match &runtime_metadata {
			// TODO: Bring back v13 eventually
			#[cfg(feature = "v13")]
			V13(v13) => ReducedRuntime::from_v13(v13).unwrap(),
			#[cfg(feature = "v14")]
			V14(v14) => ReducedRuntime::from_v14(v14).unwrap(),
			_ => panic!("Unsupported metadata version"),
		}
	}
}

#[cfg(test)]
mod test_reduced_runtime {
	use super::*;

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_reduce_runtime_v14_polkadot_9290() {
		use crate::differs::{
			reduced::diff_analyzer::{Compatible, DiffAnalyzer},
			test_runtimes::{get_runtime_file, Chain},
		};
		use wasm_loader::Source;
		use wasm_testbed::WasmTestBed;

		let runtime_file = get_runtime_file(Chain::Polkadot, 14, 9290).expect("Runtime file should exist");
		let _reduced_runtime: ReducedRuntime = WasmTestBed::new(&Source::File(runtime_file)).unwrap().metadata().into();
	}

	#[test]
	#[cfg(feature = "v14")]
	#[ignore = "local data"]
	fn test_reduce_runtime_get_pallet() {
		use crate::differs::{
			reduced::diff_analyzer::{Compatible, DiffAnalyzer},
			test_runtimes::{get_runtime_file, Chain},
		};
		use wasm_loader::Source;
		use wasm_testbed::WasmTestBed;

		let runtime_file = get_runtime_file(Chain::Polkadot, 14, 9290).expect("Runtime file should exist");
		let reduced_runtime: ReducedRuntime = WasmTestBed::new(&Source::File(runtime_file)).unwrap().metadata().into();
		assert_eq!(0_u32, reduced_runtime.get_pallet_by_name("System").unwrap().index);
		assert_eq!(1_u32, reduced_runtime.get_pallet_by_name("Scheduler").unwrap().index);
	}
}

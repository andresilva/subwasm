use super::{
	prelude::*,
	signature::{Arg, Signature},
};
use comparable::Comparable;
use serde::Serialize;
use std::fmt::Display;

/// Reduced Event
#[derive(Debug, PartialEq, Eq, Serialize, Hash, Comparable, PartialOrd, Ord)]
pub struct Event {
	index: Index,
	name: String,
	signature: Signature,

	#[comparable_ignore]
	docs: Documentation,
}

impl Display for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_fmt(format_args!("[{: >2}] {} ( {} )", self.index, self.name, self.signature));

		Ok(())
	}
}

pub fn variant_to_events(td: &TypeDefVariant<PortableForm>) -> Vec<PalletItem> {
	td.variants()
		.iter()
		.map(|vv| {
			let args = vv
				.fields()
				.iter()
				.map(|f| Arg {
					name: f.name().unwrap_or(&String::from("")).into(),
					ty: f.type_name().unwrap_or(&String::from("")).into(),
				})
				.collect();

			PalletItem::Event(Event {
				index: vv.index() as u32,
				name: vv.name().to_string(),
				signature: Signature { args },
				docs: vv.docs().iter().map(|f| f.into()).collect(),
			})
		})
		.collect()
}
use super::calls::{call::*, constant::*, error::*, event::*, storage::*};
use comparable::Comparable;
use frame_metadata::PalletCallMetadata;
use scale_info::form::PortableForm;
use std::fmt::Display;

#[derive(Debug, PartialEq, Hash, Comparable, PartialOrd, Ord, Eq)]
/// A [PalletItem] is what [ReducedRuntime](super::reduced_runtime::ReducedRuntime) are made of.
pub enum PalletItem {
	Call(Call),
	Event(Event),
	Error(Error),
	Storage(Storage),
	Constant(Constant),
}

impl Display for PalletItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		const WIDTH: usize = 9;
		match self {
			PalletItem::Call(c) => f.write_fmt(format_args!("{:<WIDTH$}: {}", "Call", c)),
			PalletItem::Event(e) => f.write_fmt(format_args!("{:<WIDTH$}: {}", "Event", e)),
			PalletItem::Error(e) => f.write_fmt(format_args!("{:<WIDTH$}: {}", "Error", e)),
			PalletItem::Constant(c) => f.write_fmt(format_args!("{:<WIDTH$}: {}", "Constant", c)),
			PalletItem::Storage(s) => f.write_fmt(format_args!("{:<WIDTH$}: {}", "Storage", s)),
		}
	}
}

impl From<&PalletCallMetadata<PortableForm>> for Call {
	fn from(_: &PalletCallMetadata<PortableForm>) -> Self {
		todo!("From<&PalletCallMetadata<PortableForm>> for Call")
	}
}

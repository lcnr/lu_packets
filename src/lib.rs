/*!
	Documentation and (de-)serialization support for LU's network protocol.
*/
#![feature(specialization)]
#![allow(incomplete_features)]


use std::fmt::Debug;
use std::io::{Result as Res, Write};

use endio::{LE, Serialize};
use endio_bit::{BEBitWriter};
use lu_packets_derive::ReplicaSerde;

trait ReplicaS<W: Write> {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()>;
}

impl<'a, W: Write, T> ReplicaS<W> for &'a T
where
	&'a T: Serialize<LE, BEBitWriter<W>>,
{
	default fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		Serialize::serialize(self, writer)
	}
}


impl<'a, W: Write> ReplicaS<W> for &'a bool {
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		writer.write_bit(*self)
	}
}

impl<W: Write, T> ReplicaS<W> for &Option<T>
where
	for<'a> &'a T: ReplicaS<W> + Serialize<LE, BEBitWriter<W>>,
{
	fn serialize(self, writer: &mut BEBitWriter<W>) -> Res<()> {
		writer.write_bit(self.is_some())?;
		if let Some(x) = self {
			ReplicaS::serialize(x, writer)?;
		}
		Ok(())
	}
}

pub trait ComponentConstruction: Debug {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()>;
}

pub trait ComponentSerialization: Debug {
	fn ser(&self, writer: &mut BEBitWriter<Vec<u8>>) -> Res<()>;
}

pub trait ComponentProtocol {
	type Construction: ComponentConstruction;
	type Serialization: ComponentSerialization;
}

pub trait ReplicaContext {}

#[derive(Debug, PartialEq, ReplicaSerde)]
pub struct ParentInfo {
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ChildInfo {}

#[derive(Debug, PartialEq)]
pub struct ParentChildInfo {
	pub parent_info: Option<ParentInfo>,
	pub child_info: Option<ChildInfo>,
}

#[derive(Debug)]
pub struct ReplicaConstruction {}

impl PartialEq<ReplicaConstruction> for ReplicaConstruction {
	fn eq(&self, rhs: &ReplicaConstruction) -> bool {
		// hacky but i don't know a better way
		format!("{:?}", self) == format!("{:?}", rhs)
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a ReplicaConstruction {
	fn serialize(self, _writer: &mut W) -> Res<()> {
		todo!()
	}
}

#[derive(Debug)]
pub struct ReplicaSerialization {
	pub network_id: u16,
	pub parent_child_info: Option<ParentChildInfo>,
	pub components: Vec<Box<dyn ComponentSerialization>>,
}

impl PartialEq<ReplicaSerialization> for ReplicaSerialization {
	fn eq(&self, rhs: &ReplicaSerialization) -> bool {
		// hacky but i don't know a better way
		format!("{:?}", self) == format!("{:?}", rhs)
	}
}

impl<'a, W: Write> Serialize<LE, W> for &'a ReplicaSerialization {
	fn serialize(self, _writer: &mut W) -> Res<()> {
		todo!()
	}
}

#[cfg(test)]
#[derive(Debug)]
pub(super) struct DummyContext<'a> {
	pub(super) inner: &'a mut &'a [u8],
}

#[cfg(test)]
impl Read for DummyContext<'_> {
	fn read(&mut self, buf: &mut [u8]) -> Res<usize> {
		Read::read(self.inner, buf)
	}
}

#[cfg(test)]
impl ReplicaContext for DummyContext<'_> {
	fn get_comp_constructions<R: Read>(&mut self, _network_id: u16, _lot: Lot, _config: &Option<LuNameValue>) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentConstruction>>> {
		vec![]
	}

	fn get_comp_serializations<R: Read>(&mut self, _network_id: u16) -> Vec<fn(&mut BEBitReader<R>) -> Res<Box<dyn ComponentSerialization>>> {
		vec![]
	}
}

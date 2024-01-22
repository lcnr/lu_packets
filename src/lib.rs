/*!
	Documentation and (de-)serialization support for LU's network protocol.
*/
#![feature(specialization)]
#![allow(incomplete_features)]
use std::io::{Result as Res, Write};

use endio::{LE, Serialize};
use endio_bit::{BEBitWriter};

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

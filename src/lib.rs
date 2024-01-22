#![feature(specialization)]
#![allow(incomplete_features)]
use std::io::Write;

use endio::{Serialize, LE};
use endio_bit::BEBitWriter;

trait ReplicaS<W: Write> {
    fn serialize(self);
}

impl<'a, W: Write, T> ReplicaS<W> for &'a T
where
    &'a T: Serialize<LE, BEBitWriter<W>>,
{
    default fn serialize(self) {}
}

impl<W: Write, T> ReplicaS<W> for &Option<T>
where
    for<'a> &'a T: ReplicaS<W> + Serialize<LE, BEBitWriter<W>>,
{
    fn serialize(self) {}
}

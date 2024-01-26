#![feature(specialization)]
#![allow(incomplete_features)]
use std::io::Write;

use endio::{Serialize, LE};

struct BitWriter<W: Write> {
    field: W,
}

impl<W: Write> Write for BitWriter<W> {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

trait ReplicaS<W: Write> {
    fn serialize(self);
}

impl<'a, W: Write, T> ReplicaS<W> for &'a T
where
    &'a T: Serialize<LE, BitWriter<W>>,
{
    default fn serialize(self) {}
}

impl<W: Write, T> ReplicaS<W> for &Option<T>
where
    for<'a> &'a T: ReplicaS<W> + Serialize<LE, BitWriter<W>>,
{
    fn serialize(self) {}
}

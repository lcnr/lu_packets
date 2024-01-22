/*!
	Documentation and (de-)serialization support for LU's network protocol.
*/
#![feature(specialization)]
#![allow(incomplete_features)]
use std::io::{Write};

use endio::{LE, Serialize};

trait ReplicaS<W: Write> {}

impl<'a, W: Write, T> ReplicaS<W> for &'a T
where
	&'a T: Serialize<LE, BEBitWriter<W>>,
{}

impl<W: Write, T> ReplicaS<W> for &Option<T>
where
	for<'a> &'a T: ReplicaS<W> + Serialize<LE, BEBitWriter<W>>,
{}

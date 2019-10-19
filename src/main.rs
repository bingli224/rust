
//extern crate timeline::{Timeline, Range};
mod timeline;
use timeline::{
	Timeline,
	Range,
};

fn main() {
/*
	let mut timeline = Timeline { ranges: Vec::new ( ) };

	timeline.add ( &[
		Range { left: 0, right: 6 },
		Range { left: 3, right: 10 },
		Range { left: 15, right: 17 },
		Range { left: 19, right: 20 },
		Range { left: 30, right: 33 },
		Range { left: 25, right: 25 },
		Range { left: 18, right: 21 },
		Range { left: 18, right: 25 },
		Range { left: -18, right: 5 },
		Range { left: 0, right: 29 },
		Range { left: -50, right: 50 },
	] );

	print ! ( "{:?}", timeline );
*/

	let mut timeline = Timeline { ranges: Vec::new ( ) };

	timeline.add ( &[
		Range { left: 1, right: 5 },
		Range { left: 12, right: 15 },
		Range { left: 42, right: 44 },
		Range { left: 70, right: 72 },
		Range { left: 36, right: 36 },
		Range { left: -4, right: 2 },
		Range { left: 43, right: 69 },
		Range { left: 15, right: 24 },
	] );

	print ! ( "{:?}", timeline );
}



/**
 * By BingLi224
 * 19:58 THA 13/10/2019
 *
 * Timeline.
 */

use std::fmt;

pub struct Range {
    pub left : i128,
    pub right : i128,
}

pub struct Timeline {
    pub ranges : Vec <Range>,
}

impl Timeline {
    //fn add ( &self, lefts : &[i128], rights : &[i128] ) {
    pub fn add ( &mut self, ranges : &[Range] ) {
		for r in ranges {
			println ! ( "in: {}/{}", r.left, r.right );
			let mut right_pos = self.ranges.len ( );
			if right_pos <= 0usize {
				self.ranges.push ( Range { left: r.left, right: r.right } );
			} else {
				let mut left_pos = 0usize;
				let mut pos = ( right_pos - left_pos ) / 2;
				//if r.left < self.ranges [ pos ]
				println ! ( "\t{}-{}", left_pos, right_pos );
				loop {
					if self.ranges [ pos ].left > r.left {
						if pos <= left_pos || self.ranges [ pos - 1 ].right < r.left {
							// new range
							self.ranges.insert ( pos, Range { left : r.left, right : r.left } );
							left_pos = pos;
							break;
						}

						right_pos = pos;
						pos = left_pos + ( pos - left_pos ) / 2;
					} else if self.ranges [ pos ].right < r.left {
						if pos >= self.ranges.len ( ) - 1 {
							self.ranges.push (
								Range { left: r.left, right: r.right }
							);
							left_pos = self.ranges.len ( ) - 1;
							break;
						} else if r.left < self.ranges [ pos + 1 ].left {
							self.ranges.insert ( pos + 1, Range { left: r.left, right: r.left } );
							left_pos = pos + 1;
							break;
						}
						
						left_pos = pos + 1;
						pos = left_pos + ( right_pos - left_pos ) / 2;
					} else {
						// in this range
						left_pos = pos;
						break;
					}
				};

				println ! ( "{:?}", self );
				println ! ( "\t{}-{}-{}", left_pos, pos, right_pos );

				let tmp_left_pos = left_pos;

				right_pos = self.ranges.len ( );
				pos = left_pos + ( right_pos - left_pos ) / 2;

				println ! ( "\t\t[{} {}]{}-[{} {}]{}-{}[{} {}]",
					self.ranges [ left_pos ].left, self.ranges [ left_pos ].right, left_pos,
					self.ranges [ pos ].left, self.ranges [ pos ].right, pos,
					right_pos, self.ranges [ right_pos - 1 ].left, self.ranges [ right_pos - 1 ].right
				);

				loop {
					if self.ranges [ pos ].left > r.right {
						if pos <= left_pos {
							// NOTE: because input.left > input.right
							self.ranges [ left_pos ].right = r.right;
						} else if self.ranges [ pos - 1 ].right < r.right {
							// new range
							pos -= 1;
							self.ranges [ pos ].right = r.right;
							right_pos = pos;
							break;
						}

						right_pos = pos;
						pos = left_pos + ( pos - left_pos ) / 2;
					} else if self.ranges [ pos ].right < r.right {
						if pos >= self.ranges.len ( ) - 1 {
							self.ranges.push (
								Range { left: r.right, right: r.right }
							);
							right_pos = self.ranges.len ( ) - 1;
							break;
						} else if r.right < self.ranges [ pos + 1 ].left {
							right_pos = pos + 1;
							self.ranges.insert ( right_pos, Range { left: r.right, right: r.right } );
							break;
						}
						
						left_pos = pos + 1;
						pos = left_pos + ( right_pos - left_pos ) / 2;
					} else {
						// in this range
						right_pos = pos;
						break;
					}
				};

				println ! ( "{:?}", self );

				let tmp_right = self.ranges [ right_pos ].right;

				println ! ( "\t\t[{} {}]{}-{}[{} {}]",
					self.ranges [ tmp_left_pos ].left, self.ranges [ tmp_left_pos ].right, tmp_left_pos,
					right_pos, self.ranges [ right_pos ].left, self.ranges [ right_pos ].right
				);

				while tmp_left_pos < right_pos {
					self.ranges.remove ( right_pos );
					right_pos -= 1;
				}
				
				if self.ranges [ tmp_left_pos ].right < tmp_right {
					self.ranges [ tmp_left_pos ].right = tmp_right;
				}
			}

			println ! ( "{:?}", self );
			println ! ( " -----next------" );
        }
    }
}

impl fmt::Debug for Timeline {
    fn fmt ( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		let mut res : fmt::Result = Ok ( ( ) );
        for r in self.ranges.iter ( ) {
            res = write ! ( f, "{} {}|", r.left, r.right );
            if res.is_err ( ) {
                return res;
			}
        }
		res
    }
}


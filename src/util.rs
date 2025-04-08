use core::{iter::Peekable, num::TryFromIntError, str::Chars};

use crate::{consts::msg::FAILED_CONVERTING_TO_INDEPENDENT_BITS, error::SpannedError};

// pub fn print_error_chain(mut err: &(dyn core::error::Error), fn:T) {
//   eprintln!("{}", err); // top-level error
//   while let Some(source) = err.source() {
//       eprintln!("→ caused by: {}", source);
//       err = source;
//   }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointInfo {
  pub line: u32,
  pub column: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointLength<'a> {
  pub content: &'a str,
  pub index: u32,
}
impl<'a> PointLength<'a> {
  pub fn new(content: &'a str) -> Self {
    Self {
      content,
      index: 0,
    }
  }
  pub fn increment(&mut self) {
    self.index += 1;
  }
  pub fn to_point_info(&self) -> PointInfo { 
    return self.try_to_point_info().expect(FAILED_CONVERTING_TO_INDEPENDENT_BITS);
  }
  pub fn try_to_point_info(&self) -> Result<PointInfo, TryFromIntError> {
    let mut line: u32 = 1;
    let mut column: u32 = 0;
    for c in self.content.chars().take(self.index.try_into()?)  {
      if c == '\n' {
        line += 1;
        column = 0;
      } else {
        column += 1;
        if c == '\t' {
          column += 3;
        }
      }
    }
    return Ok(PointInfo {
      line,
      column
    })
  }
}
pub struct PeekableWithPoint<'a> {
  chars:Peekable<Chars<'a>>, 
  pub point:PointLength<'a>,
}
impl<'a> PeekableWithPoint<'a> {
  pub fn new(content: &'a str) -> Self {
    return Self {
      chars:content.chars().peekable(),
      point:PointLength::new(content)
    }
  }
  pub fn peek(&mut self)->Option<&char>{
    return self.chars.peek();
  }
  pub fn as_spanned_error<T>(&self, error:T) -> SpannedError<T> {
    // andddd its looping, very great
    //
    // return SpannedError::new(
    //   error,
    //   if let Ok(v) = self.point.try_to_point_info() {
    //     v
    //   } else {
    //     return Err(self.as_spanned_error(LexError::Runtime(
    //         RuntimeError::FailedConvertingToIndependentBits,
    //     )))
    //   }
    // )
    return SpannedError::new(
      error,
      self.point.to_point_info()
    )
  }
      // a fallback for 16bit platforms, even tho we dosent support it
      pub fn num_into_usize<T:TryInto<usize>>(&self, num:T) -> usize {
        let start_raw: Result<usize, T::Error> = num.try_into();
        if let Ok(v) = start_raw {
            return v;
        } else {
            panic!("{}", FAILED_CONVERTING_TO_INDEPENDENT_BITS);
        }
    }
}
impl<'a> Iterator for PeekableWithPoint<'a> {
  type Item = char;
  fn next(&mut self) -> Option<Self::Item> {
    let res = self.chars.next();
    if let Some(ch) = res  {
      self.point.increment();
      return Some(ch);
    } else {
      return None;
    }
  }
}
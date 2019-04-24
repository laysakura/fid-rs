use super::{Fid, FidIter};

impl<'iter> Fid {
    /// Creates an iterator over FID's bit vector.
    ///
    /// # Examples
    /// ```
    /// use fid_rs::Fid;
    ///
    /// let fid = Fid::from("1010_1010");
    /// for (i, bit) in fid.iter().enumerate() {
    ///     assert_eq!(bit, fid[i as u64]);
    /// }
    /// ```
    pub fn iter(&'iter self) -> FidIter<'iter> {
        FidIter { fid: self, i: 0 }
    }
}

impl<'iter> Iterator for FidIter<'iter> {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.fid.len() {
            None
        } else {
            self.i += 1;
            Some(self.fid[self.i - 1])
        }
    }
}

#[cfg(test)]
mod iter_success_tests {
    use crate::Fid;

    #[test]
    fn iter() {
        let fid = Fid::from("1010_1010");
        for (i, bit) in fid.iter().enumerate() {
            assert_eq!(bit, fid[i as u64]);
        }
    }
}

#[cfg(test)]
mod iter_failure_tests {
    // Nothing to test
}

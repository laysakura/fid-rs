use super::{Louds, LoudsIndex, LoudsNodeNum};

impl Louds {
    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn node_num_to_index(&self, node_num: &LoudsNodeNum) -> LoudsIndex {
        assert!(node_num.value() > 0);

        let index = self.lbs.select(node_num.value()).expect(&format!(
            "NodeNum({}) does not exist in this LOUDS",
            node_num.value(),
        ));
        LoudsIndex::new(index)
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    pub fn index_to_node_num(&self, index: &LoudsIndex) -> LoudsNodeNum {
        self.validate_index(&index);

        let node_num = self.lbs.rank(index.value());
        LoudsNodeNum::new(node_num)
    }

    /// # Panics
    /// - `index` does not point to any node in this LOUDS.
    /// - `index == 0`: (node#1 is root and doesn't have parent)
    pub fn child_to_parent(&self, index: &LoudsIndex) -> LoudsNodeNum {
        self.validate_index(&index);
        assert!(index.value != 0, "node#1 is root and doesn't have parent");

        let parent_node_num = self.lbs.rank0(index.value());
        LoudsNodeNum::new(parent_node_num)
    }

    /// # Panics
    /// `node_num` does not exist in this LOUDS.
    pub fn parent_to_children(&self, node_num: &LoudsNodeNum) -> Vec<LoudsIndex> {
        assert!(node_num.value() > 0);

        let parent_start_index = self.lbs.select0(node_num.value()).expect(&format!(
            "NodeNum({}) does not exist in this LOUDS",
            node_num.value(),
        )) + 1;

        let mut children_index: Vec<u64> = vec![];
        let mut i = parent_start_index;
        loop {
            if self.lbs.access(i) == false {
                break;
            } else {
                children_index.push(i);
            }
            i += 1;
        }

        children_index.iter().map(|i| LoudsIndex::new(*i)).collect()
    }

    /// # Panics
    /// `index` does not point to any node in this LOUDS.
    fn validate_index(&self, index: &LoudsIndex) {
        assert_eq!(
            self.lbs.access(index.value()),
            true,
            "LBS[index={:?}] must be '1'",
            index,
        );
    }
}

#[cfg(test)]
mod node_num_to_index_success_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_index) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let index = louds.node_num_to_index(&LoudsNodeNum::new(node_num));
                assert_eq!(index, LoudsIndex::new(expected_index));
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, 0),

        t2_1: ("10_10_0", 1, 0),
        t2_2: ("10_10_0", 2, 2),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 2),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 3),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 4),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, 6),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 9),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, 10),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, 11),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 15),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 17),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 18),
    }

}

#[cfg(test)]
mod node_num_to_index_failure_tests {
    use crate::{BitString, LoudsBuilder, LoudsNodeNum};

    macro_rules! parameterized_node_not_found_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, node_num) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.node_num_to_index(&LoudsNodeNum::new(node_num));
            }
        )*
        }
    }

    parameterized_node_not_found_tests! {
        t1_1: ("10_0", 0),
        t1_2: ("10_0", 2),

        t2_1: ("10_10_0", 0),
        t2_2: ("10_10_0", 3),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
    }
}

#[cfg(test)]
mod index_to_node_num_success_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index, expected_node_num) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let node_num = louds.index_to_node_num(&LoudsIndex::new(index));
                assert_eq!(node_num, LoudsNodeNum::new(expected_node_num));
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 0, 1),

        t2_1: ("10_10_0", 0, 1),
        t2_2: ("10_10_0", 2, 2),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0, 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 2),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 3),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 4),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 5),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 6),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 7),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 8),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 15, 9),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 17, 10),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 18, 11),
    }

}

#[cfg(test)]
mod index_to_node_num_failure_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex};

    macro_rules! parameterized_index_not_point_to_node_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, index) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.index_to_node_num(&LoudsIndex::new(index));
            }
        )*
        }
    }

    parameterized_index_not_point_to_node_tests! {
        t1_1: ("10_0", 1),
        t1_2: ("10_0", 3),

        t2_1: ("10_10_0", 1),
        t2_2: ("10_10_0", 3),
        t2_3: ("10_10_0", 4),
        t2_4: ("10_10_0", 5),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 13),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 14),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 16),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 19),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 20),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 21),
        t3_12: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 22),
        t3_13: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 23),
        t3_14: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 24),
    }
}

#[cfg(test)]
mod child_to_parent_success_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, index, expected_parent) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let parent = louds.child_to_parent(&LoudsIndex::new(index));
                assert_eq!(parent, LoudsNodeNum::new(expected_parent));
            }
        )*
        }
    }

    parameterized_tests! {
        t2_1: ("10_10_0", 2, 1),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, 1),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, 1),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, 2),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, 4),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, 4),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, 4),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 15, 7),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 17, 8),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 18, 8),
    }
}

#[cfg(test)]
mod child_to_parent_failure_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex};

    macro_rules! parameterized_index_not_point_to_node_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, index) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.child_to_parent(&LoudsIndex::new(index));
            }
        )*
        }
    }

    parameterized_index_not_point_to_node_tests! {
        t1_1: ("10_0", 1),
        t1_2: ("10_0", 3),

        t2_1: ("10_10_0", 1),
        t2_2: ("10_10_0", 3),
        t2_3: ("10_10_0", 4),
        t2_4: ("10_10_0", 5),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 13),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 14),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 16),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 19),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 20),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 21),
        t3_12: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 22),
        t3_13: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 23),
        t3_14: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 24),
    }

    macro_rules! parameterized_root_not_have_parent_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let in_s = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.child_to_parent(&LoudsIndex::new(0));
            }
        )*
        }
    }

    parameterized_root_not_have_parent_tests! {
        t1: "10_0",
        t2: "10_10_0",
        t3: "10_1110_10_0_1110_0_0_10_110_0_0_0",
    }
}

#[cfg(test)]
mod parent_to_children_success_tests {
    use crate::{BitString, LoudsBuilder, LoudsIndex, LoudsNodeNum};

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (in_s, node_num, expected_children) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let children = louds.parent_to_children(&LoudsNodeNum::new(node_num));
                assert_eq!(children, expected_children.iter().map(|c| LoudsIndex::new(*c)).collect::<Vec<LoudsIndex>>());
            }
        )*
        }
    }

    parameterized_tests! {
        t1_1: ("10_0", 1, vec!()),

        t2_1: ("10_10_0", 1, vec!(2)),
        t2_2: ("10_10_0", 2, vec!()),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 1, vec!(2, 3, 4)),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 2, vec!(6)),
        t3_3: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 3, vec!()),
        t3_4: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 4, vec!(9, 10, 11)),
        t3_5: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 5, vec!()),
        t3_6: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 6, vec!()),
        t3_7: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 7, vec!(15)),
        t3_8: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 8, vec!(17, 18)),
        t3_9: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 9, vec!()),
        t3_10: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 10, vec!()),
        t3_11: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 11, vec!()),
    }

}

#[cfg(test)]
mod parent_to_children_failure_tests {
    use crate::{BitString, LoudsBuilder, LoudsNodeNum};

    macro_rules! parameterized_node_not_found_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $name() {
                let (in_s, node_num) = $value;
                let bs = BitString::new(in_s);
                let louds = LoudsBuilder::from_bit_string(bs).build();
                let _ = louds.parent_to_children(&LoudsNodeNum::new(node_num));
            }
        )*
        }
    }

    parameterized_node_not_found_tests! {
        t1_1: ("10_0", 0),
        t1_2: ("10_0", 2),

        t2_1: ("10_10_0", 0),
        t2_2: ("10_10_0", 3),

        t3_1: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 0),
        t3_2: ("10_1110_10_0_1110_0_0_10_110_0_0_0", 12),
    }
}

mod louds_feature_test {
    use succinct_rs::{BitString, LoudsBuilder, LoudsNodeNum};

    #[test]
    fn fuzzing_test() {
        use rand::prelude::*;

        let samples = 100;
        let mut rng = rand::thread_rng();

        fn generate_lbs(rng: &mut ThreadRng) -> BitString {
            let mut s = String::from("10");
            let (mut cnt0, mut cnt1) = (1u64, 1u64);
            while cnt0 < cnt1 + 1 {
                let r = rng.gen::<f64>();
                if r < 0.6 {
                    s = format!("{}{}", s, "0");
                    cnt0 += 1;
                } else {
                    s = format!("{}{}", s, "1");
                    cnt1 += 1;
                }
            }
            BitString::new(&s)
        }

        for _ in 0..samples {
            let bs = generate_lbs(&mut rng);
            eprintln!("build(): LBS = \"{}\"", bs.str());

            let n_nodes = bs.str().len() / 2;
            let louds = LoudsBuilder::from_bit_string(bs).build();

            for raw_node_num in 1..=n_nodes {
                let node_num = LoudsNodeNum::new(raw_node_num as u64);
                eprintln!("NodeNum({:?})", raw_node_num);

                // index(node_num_to_index(node_num)) == node_num
                let index = louds.node_num_to_index(&node_num);
                assert_eq!(louds.index_to_node_num(&index), node_num);

                // `node_num`'s children have `node_num` as parent.
                for child_index in louds.parent_to_children(&node_num) {
                    assert_eq!(louds.child_to_parent(&child_index), node_num);
                }
            }
        }
    }
}

pub mod vectos {
    pub fn vectors_sample() {
        let mut v: Vec<i32> = Vec::new();
        v.push(5);

        match v.get(0) {
            Some(value) => println!("v::: {}", value),
            None => println!("No value found"),
        }

        // println!("v: {}", v.get(0).unwrap());
    }

    pub fn vectors_sample2() -> Vec<i32> {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5];
        return v;
    }
}

pub mod hash_map {
    pub fn hash_map_sample() {
        use std::collections::HashMap;

        let v_1 = "v_1".to_string();
        let v_2 = "v_2".to_string();

        let mut map = HashMap::new();
        map.insert(v_1, 1);
        map.insert(v_2, 2);

        // println!("v_1: {v_1}, v_2: {v_2}"); //! This will not work
    }
}

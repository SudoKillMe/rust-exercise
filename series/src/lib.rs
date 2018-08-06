pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result = vec![];
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    let vec: Vec<&str> = digits
                        .split("")
                        .filter(|x| !x.is_empty())
                        .collect();
    let mut iter = match vec.windows(len) {
        Ok(val) => val,
        Err(error) => vec!["".to_string(); digits.len() + 1]
    };



    while true {
        let sub_vec = iter.next();
        match sub_vec {
            Some(v) => result.push(v.join("")),
            None => break
        }
    }

    result

}

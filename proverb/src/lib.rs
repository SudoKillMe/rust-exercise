pub fn build_proverb(list: Vec<&str>) -> String {
    // let mut ite = list.iter();
    // let mut current = ite.next();
    // let mut result = Vec::new();
    // while current != None {
    //     let current_value = match current {
    //         Some(v) => v,
    //         _ => "",
    //     };

    //     current = ite.next();

    //     let str = match current {
    //         Some(v) => format!("For want of a {} the {} was lost.", current_value, v),
    //         None => format!("And all for the want of a {}.", list[0]),
    //     };
    //     result.push(str);
    // }

    // result.join("\n")
    list.iter()
        .fold("", |acc, x| {
            let res = match x {
                Some(v) => format!("For want of a"),
            }
        })
}

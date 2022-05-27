
#[allow(unused)]
#[allow(unused_assignments)]
macro_rules! Jsonify {
    ($($key:expr => $val:expr),*) => {
        {
            let mut result: String=String::from("{");
        let mut add_comma = false;
        $(
            if (add_comma){
                result.push_str(format!(", \"{}\": \"{}\"", $key, $val).clone().as_str());
            } else {
                result.push_str(format!("\"{}\": \"{}\"", $key, $val).clone().as_str());
                add_comma = true;
            }
        )*
        result.push_str("}");
        result
    }
    };
}

use std::collections::VecDeque;

pub fn shunting_yard(tokens: Vec<String>) -> Vec<String> {
    let mut st: Vec<String> = Vec::new();
    let mut q = VecDeque::new();

    for token in tokens {
        if let Ok(_) = token.parse::<i32>() {
            q.push_back(token);
            continue
        }

        match token.as_str() {
            "+" | "-" => {
                while let Some(op) = st.last() {
                    if op.as_str() == "(" { break }
                    q.push_back(op.to_string());
                    st.pop();
                }
                st.push(token);
            },
            "(" => st.push(token),
            ")" => {
                if st.is_empty() { unreachable!() }
                while let Some(op) = st.pop() {
                    if op == "(" { break }
                    else { q.push_back(op); }
                }
            },
            _ => (),
        }
    }

    while let Some(op) = st.pop() {
        if op.as_str() == "(" { unreachable!() }
        q.push_back(op);
    }

    Vec::from(q)
}

use convert_case::{ Case, Casing };

pub fn expected_variable(cmp: &str, exp: &str) -> Option<String> {
    let cl = cmp.to_lowercase();
    let el = exp.to_lowercase();

    if cl != cl.to_case(Case::Camel) && cl != cl.to_case(Case::Snake) {
        return None;
    }

    let dist = edit_distance(&cl, &el);
    let max_len = std::cmp::max(cl.len(), el.len());

    if max_len == 0 {
        return Some("100%".to_string());
    }

    let pct = 100 - (dist as isize * 100 / exp.len() as isize);
    if pct > 50 {
        return Some(format!("{}%", pct));
    } else {
        return None;
    }
}

pub fn edit_distance(src: &str, tgt: &str) -> usize {
    let s: Vec<char> = src.chars().collect();
    let t: Vec<char> = tgt.chars().collect();

    let ls = s.len();
    let lt = t.len();

    let mut m = vec![vec![0; lt + 1]; ls + 1];

    for i in 0..=ls {
        m[i][0] = i;
    }

    for j in 0..=lt {
        m[0][j] = j;
    }

    for i in 1..=ls {
        for j in 1..=lt {
            let cost = if s[i - 1] == t[j - 1] { 0 } else { 1 };
            let ins = m[i - 1][j] + 1;
            let del = m[i][j - 1] + 1;
            let rep = m[i - 1][j - 1] + cost;
            m[i][j] = std::cmp::min(std::cmp::min(ins, del), rep);
        }
    }
    
    m[ls][lt]
}
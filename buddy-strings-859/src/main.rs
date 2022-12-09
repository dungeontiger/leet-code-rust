pub fn main() {

}

pub fn buddy_strings(s: String, goal: String) -> bool {
    let mut s_mut = s.clone();
    for i in 0..s.len() {
        for j in i+1..s_mut.len() {
            if i != j {
                // save originals
                let org_i = s.get(i..i+1).unwrap();
                let org_j = s.get(j..j+1).unwrap();
                // replace the i character with the j character
                s_mut.replace_range(i..i+1, org_j);
                // replace the j character with the i character
                s_mut.replace_range(j..j+1, org_i);
                // see if this equals goal
                if (s_mut == goal) {
                    return true;
                }
                s_mut.replace_range(i..i+1, org_i);
                s_mut.replace_range(j..j+1, org_j);
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(buddy_strings("ab".to_string(), "ba".to_string()));
    }

    #[test]
    fn example2() {
        assert!(!buddy_strings("ab".to_string(), "ab".to_string()));
    }

    #[test]
    fn example3() {
        assert!(buddy_strings("aa".to_string(), "aa".to_string()));
    }
}


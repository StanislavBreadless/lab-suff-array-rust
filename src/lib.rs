pub fn build_suffix_array(s: String) -> Vec<usize> {
    let s = format!("{}{}", s, "\x01");
    construct_suffix_array(&s).into_iter().skip(1).collect()
}

fn wrapped_pos(x: usize, len: usize) -> usize {
    x % len
}

fn get_eq_classes(prev_eq_classes: &Vec<usize>, substr_len: usize) -> Vec<usize> {
    // Array where the new eq classes will be put
    let mut eq_classes: Vec<usize> = vec![0; prev_eq_classes.len()];

    // Triples of (pos, eq_class of suffix starting at pos, eq_class of suffix starting at pos + substr_len)
    let mut triples: Vec<(usize, usize, usize)> = vec![];

    for i in 0..prev_eq_classes.len() {
        let first = prev_eq_classes[i];
        let second = prev_eq_classes[wrapped_pos(i + substr_len, prev_eq_classes.len())];

        triples.push((i, first, second));
    }

    // First round of radix sort: sort by the last param of the triple
    let mut radix_sort1: Vec<Vec<(usize,usize,usize)>> = vec![vec![]; std::cmp::max(prev_eq_classes.len(), 256)];
    for i in 0..prev_eq_classes.len() {
        radix_sort1[triples[i].2].push(triples[i]);
    }

    // The second round of radix sort: sort by the second param of the triple
    let mut radix_sort2: Vec<Vec<(usize,usize,usize)>> = vec![vec![]; std::cmp::max(prev_eq_classes.len(), 256)];
    for i in radix_sort1.iter() {
        for j in i.iter() {
            radix_sort2[j.1].push(*j);
        }
    }

    // Now we need to assign the same eq classes for the same suffixes
    let mut eq_class_id = 0;
    for i in radix_sort2.iter() {
        if i.len() == 0 {
            continue;
        }

        let mut prev = i[0];
        eq_classes[prev.0] = eq_class_id;
        for j in i.iter().skip(1) {
            if (j.1, j.2) != (prev.1, prev.2) {
                eq_class_id += 1;
            }
            prev = *j;
            eq_classes[j.0] = eq_class_id;
        }
        eq_class_id += 1;
    }

    eq_classes
}

fn construct_suffix_array(s: &str) -> Vec<usize> {
    for c in s.chars() {
        if !c.is_ascii() {
            panic!("Input string must be ASCII");
        }
    }

    let mut eq_classes: Vec<usize> = s.chars().map(|c| c as usize).collect();

    // Using log(n) rounds 
    let mut k = 1;
    while k < s.len() {
        eq_classes = get_eq_classes(&eq_classes, k);
        k *= 2;
    }

    let mut eq_class_to_pos: Vec<Vec<usize>> = vec![vec![]; s.len()];

    for i in 0..eq_classes.len() {
        eq_class_to_pos[eq_classes[i]].push(i);
    }

    let mut result = vec![];

    for i in eq_class_to_pos.iter() {
        for j in i.iter() {
            result.push(*j);
        }
    }

    result
} 

pub fn common(str1: &str, str2: &str, str3: &str) -> usize {
    let mut alpha1 = [false; 52];
    for ele in str1.chars() {
        if ele >= 'a' && ele <= 'z' {
            let index = ele as u32 - 'a' as u32;
            alpha1[index as usize] = true;
        } else if ele >= 'A' && ele <= 'Z' {
            let index = ele as u32 - 'A' as u32 + 26;
            alpha1[index as usize] = true;
        }
    }

    let mut alpha2 = [false; 52];
    for ele in str2.chars() {
        if ele >= 'a' && ele <= 'z' {
            let index = ele as u32 - 'a' as u32;
            alpha2[index as usize] = true;
        } else if ele >= 'A' && ele <= 'Z' {
            let index = ele as u32 - 'A' as u32 + 26;
            alpha2[index as usize] = true;
        }
    }

    let mut alpha3 = [false; 52];
    for ele in str3.chars() {
        if ele >= 'a' && ele <= 'z' {
            let index = ele as u32 - 'a' as u32;
            alpha3[index as usize] = true;
        } else if ele >= 'A' && ele <= 'Z' {
            let index = ele as u32 - 'A' as u32 + 26;
            alpha3[index as usize] = true;
        }
    }

    for i in 0..52 {
        if alpha1[i] && alpha2[i] && alpha3[i] {
            return i + 1;
        }
    }

    return 0;
}


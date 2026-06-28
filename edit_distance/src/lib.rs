pub fn edit_distance(source: &str, target: &str) -> usize {
    let mut matrix = vec![vec![0; source.len() + 1]; target.len() + 1];

    let source: Vec<char> = source.chars().collect();
    let target: Vec<char> = target.chars().collect();

    for i in 0..matrix[0].len() {
        matrix[0][i] = i;
    }

    for j in 0..matrix.len() {
        matrix[j][0] = j;
    }

    for y in 1..matrix.len() {
        for z in 1..matrix[y].len() {
            if target[y - 1] == source[z - 1] {
                matrix[y][z] = matrix[y - 1][z - 1];
            } else {
                matrix[y][z] = matrix[y-1][z-1].min(matrix[y][z-1].min(matrix[y-1][z]))+1
            }
        }
    }

    return matrix[matrix.len()-1][matrix[0].len()-1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let source = "alignment";
        let target = "assignment";
        
        println!(
            "It's necessary to make {} change(s) to {:?} to get {:?}",
            edit_distance(source, target),
            source,
            target
        );
    }
}

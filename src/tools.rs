pub fn parse_test_cases(s: &str) -> Vec<Vec<&str>> {
        s.trim().split('\n')
                .map(|line| line.split('|').map(|v| v.trim()).collect())
                .collect()
}

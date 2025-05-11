

/// Parses a comma-separated list of TCP ports and port ranges into a `Vec<u16>`.
///
/// - Individual ports are specified as numeric literals (e.g. `"22"`).
/// - Ranges are denoted with a hyphen (e.g. `"1000-1005"` expands to 1000, 1001, …, 1005).
///
/// # Arguments
/// * `list` — an `&str` containing comma-separated port tokens or ranges (e.g. `"22,80,1000-1002"`).  
///
/// # Returns
/// * `Ok(Vec<u16>)` with all parsed ports in the order they appear (including expanded ranges).  
/// * `Err(String)` if any token:
///   - Fails to parse as a number (non-numeric input)  
///   - Falls outside the valid TCP/UDP port range (1–65535)  
///   - Specifies an invalid range (start is zero or greater than end, e.g. `"0-10"` or `"200-100"`)
pub fn parse_list(list: &str) -> Result<Vec<u16>, String> {
    list.split(',').try_fold(Vec::new(), |mut acc, token| {
        // split into either ["start","end"] or just ["port"]
        let mut parts = token.splitn(2, '-');
        let start_str = parts.next().unwrap();
        let start: u16 = start_str
            .parse()
            .map_err(|_| format!("Invalid number: {}", start_str))?;

        if let Some(end_str) = parts.next() {
            let end: u16 = end_str
                .parse()
                .map_err(|_| format!("Invalid number: {}", end_str))?;
            if start == 0 || end == 0 || start > end {
                return Err(format!("Invalid range: {}", token));
            }
            acc.extend(start..=end);
        } else {
            if start == 0 {
                return Err(format!("Invalid port: {}", token));
            }
            acc.push(start);
        }

        Ok(acc)
    })
}

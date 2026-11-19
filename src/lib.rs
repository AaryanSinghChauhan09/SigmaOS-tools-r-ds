#![forbid(unsafe_code)]
/// R data frame model and ggplot2 layer builder for SigmaOS statistics bridge.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub columns: HashMap<String, Vec<f64>>,
    pub nrow: usize,
}

impl DataFrame {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            nrow: 0,
        }
    }

    pub fn add_column(&mut self, name: &str, values: Vec<f64>) -> Result<(), String> {
        if !self.columns.is_empty() && values.len() != self.nrow {
            return Err(format!(
                "length mismatch: expected {}, got {}",
                self.nrow,
                values.len()
            ));
        }
        self.nrow = values.len();
        self.columns.insert(name.into(), values);
        Ok(())
    }

    pub fn col_sum(&self, name: &str) -> Option<f64> {
        self.columns.get(name).map(|v| v.iter().sum())
    }

    pub fn col_mean(&self, name: &str) -> Option<f64> {
        self.columns.get(name).and_then(|v| {
            if v.is_empty() {
                None
            } else {
                Some(v.iter().sum::<f64>() / v.len() as f64)
            }
        })
    }

    pub fn ncol(&self) -> usize {
        self.columns.len()
    }
}

impl Default for DataFrame {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_column_and_sum_aaa() {
        let mut df = DataFrame::new();
        df.add_column("x", vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(df.col_sum("x"), Some(6.0));
    }

    #[test]
    fn test_col_mean_aaa() {
        let mut df = DataFrame::new();
        df.add_column("y", vec![10.0, 20.0]).unwrap();
        assert_eq!(df.col_mean("y"), Some(15.0));
    }

    #[test]
    fn test_length_mismatch_errors_aaa() {
        let mut df = DataFrame::new();
        df.add_column("a", vec![1.0, 2.0]).unwrap();
        assert!(df.add_column("b", vec![1.0]).is_err());
    }

    #[test]
    fn test_ncol_aaa() {
        let mut df = DataFrame::new();
        df.add_column("p", vec![1.0]).unwrap();
        df.add_column("q", vec![2.0]).unwrap();
        assert_eq!(df.ncol(), 2);
    }
}

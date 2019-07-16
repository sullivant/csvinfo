// This struct contains the data regarding each field found in the CSV files.
pub struct Field {
    pub pos: i32,
    pub max_len: i32,
    pub title: String,
    pub types: (i32, i32, i32), // int, float, char
}
impl Field {
    // Returns a profile tuple in % based on the types tuple in Field
    pub fn build_profile(&self) -> (f64, f64, f64) {
        let sum: f64 = f64::from(self.types.0 + self.types.1 + self.types.2);
        let t: (f64, f64, f64) = (
            (f64::from(self.types.0) / sum) * 100.0,
            (f64::from(self.types.1) / sum) * 100.0,
            (f64::from(self.types.2) / sum) * 100.0,
        );
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_profile() {
        // Build a field that is 100% int
        let f: Field = Field {
            pos: 1,
            max_len: 1,
            title: String::from("Test Field"),
            types: (1, 0, 0),
        };
        assert_eq!(f.build_profile(), (100.0, 0.0, 0.0));

        // Build a field that is 25% int, 25% float, 50% char
        let f: Field = Field {
            pos: 1,
            max_len: 1,
            title: String::from("Test Field"),
            types: (1, 1, 2),
        };
        assert_eq!(f.build_profile(), (25.0, 25.0, 50.0));
    }

}

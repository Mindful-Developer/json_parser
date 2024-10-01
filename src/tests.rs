#[cfg(test)]
mod tests {
    use crate::language_tools::parser::Parser;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    // STEP 1
    #[test]
    fn test_parse_empty_object() {
        let mut file = File::open(Path::new("src/tests/step1/valid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_empty_object_fail() {
        let mut file = File::open(Path::new("src/tests/step1/invalid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_err());
    }

    // STEP 2
    #[test]
    fn test_parse_valid_string_object() {
        let mut file = File::open(Path::new("src/tests/step2/valid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_valid_multiple_string_objects() {
        let mut file = File::open(Path::new("src/tests/step2/valid2.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_valid_string_trailing_comma() {
        let mut file = File::open(Path::new("src/tests/step2/invalid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_key() {
        let mut file = File::open(Path::new("src/tests/step2/invalid2.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_err());
    }

    // STEP 3
    #[test]
    fn test_parse_valid_types() {
        let mut file = File::open(Path::new("src/tests/step3/valid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_invalid_type() {
        let mut file = File::open(Path::new("src/tests/step3/invalid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_err());
    }

    // STEP 4
    #[test]
    fn test_parse_nested_empty_objects() {
        let mut file = File::open(Path::new("src/tests/step4/valid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_nonempty_objects() {
        let mut file = File::open(Path::new("src/tests/step4/valid2.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_invalid_object() {
        let mut file = File::open(Path::new("src/tests/step4/invalid.json")).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut parser = Parser::new(&contents);
        let result = parser.parse();
        assert!(result.is_err());
    }

    // STEP 5 - ULTIMATE
    #[test]
    fn test_all_cases() {
        let path = Path::new("src/tests/step5/");
        let mut errors = vec![];
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if entry.file_name().to_str().unwrap().starts_with("pass") {
                let mut file = File::open(path.clone()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let mut parser = Parser::new(&contents);
                let result = parser.parse();
                if result.is_err() {
                    errors.push(result.err().unwrap() + "\n" + path.display().to_string().as_str() + "\n");
                }
            } else if entry.file_name().to_str().unwrap().starts_with("fail") {
                let mut file = File::open(path.clone()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                let mut parser = Parser::new(&contents);
                let result = parser.parse();
                if result.is_ok() {
                    errors.push(path.display().to_string().as_str().to_owned() + " should have failed but passed\n" +
                    &contents + "\n");
                }
            } else {
                panic!("File {} is not a test file", path.display());
            }
        }
        assert!(errors.is_empty(), "{} errors found\n{}", errors.len(), errors.join("\n"));
    }
}


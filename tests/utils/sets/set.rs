#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rust_git::utils::sets::set::{difference, idem_set_different_content};
    
    #[test]
    fn test_01_difference_between_sets() {
        let mut set = HashMap::new();
        set.insert("A".to_string(), "1".to_string());
        set.insert("B".to_string(), "2".to_string());

    
        let mut other_set = HashMap::new();
        other_set.insert("A".to_string(), "1".to_string());
        other_set.insert("C".to_string(), "2".to_string());

        let difference = difference(&set, &other_set);
        assert_eq!(difference.len(), 1)
    }
    
    #[test]
    fn test_02_difference_between_sets() {
        let mut set = HashMap::new();
        set.insert("A".to_string(), "1".to_string());
        set.insert("B".to_string(), "2".to_string());

    
        let mut other_set = HashMap::new();
        other_set.insert("A".to_string(), "1".to_string());
        other_set.insert("C".to_string(), "2".to_string());

        let difference = difference(&set, &other_set);
        assert_eq!(difference.contains_key("B"), true);
    }
    
    #[test]
    fn test_03_common_elements_with_different_content() {
        let mut set = HashMap::new();
        set.insert("A".to_string(), "1".to_string());
        set.insert("B".to_string(), "2".to_string());

    
        let mut other_set = HashMap::new();
        other_set.insert("A".to_string(), "1".to_string());
        other_set.insert("B".to_string(), "3".to_string());
        
        let result = idem_set_different_content(&set, &other_set);
        assert_eq!(result.len(), 1);
    }
    
    #[test]
    fn test_04_common_elements_with_different_content() {
        let mut set = HashMap::new();
        set.insert("A".to_string(), "1".to_string());
        set.insert("B".to_string(), "2".to_string());

    
        let mut other_set = HashMap::new();
        other_set.insert("A".to_string(), "1".to_string());
        other_set.insert("B".to_string(), "3".to_string());
        
        let result = idem_set_different_content(&set, &other_set);
        assert_eq!(result.contains_key("B"), true);
    }
    
}
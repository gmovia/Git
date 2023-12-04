use std::collections::HashMap;

// Recibe dos conjuntos.
// Devuelve la diferencia entre ambos conjuntos. Lo que esta en A pero no esta en B.

pub fn difference(
    a: &HashMap<String, String>,
    b: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    for (key, value) in a {
        if !b.contains_key(key) {
            hashmap.insert(key.to_string(), value.to_string());
        }
    }
    hashmap
}

// Recibe dos conjuntos.
// Devuelve aquellos elementos que se encuentran en ambos conjuntos pero que su contenido es diferente.

pub fn idem_set_different_content(
    a: &HashMap<String, String>,
    b: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut hashmap = HashMap::new();
    for (key, value) in a {
        if let Some(value_in_b) = b.get(key) {
            if *value != *value_in_b {
                hashmap.insert(key.to_string(), value.to_string());
            }
        }
    }
    hashmap
}

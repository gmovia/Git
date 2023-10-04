#[cfg(test)]
mod tests {
    use rust_git::{file::VSCFile, version_control_system::VersionControlSystem};
    use std::{collections::HashMap, path::Path};

    fn equals(staging_area: HashMap<String, VSCFile>, path: &Path, state: &str) -> bool {
        if let Some(value) = staging_area.get(&path.display().to_string()) {
            return value.state == state.to_string();
        }
        false
    }

    #[test]
    pub fn test_01_add_file_1_to_staging_area() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files/file1.txt");

        let staging_area = version_control_system.add(path);
        assert!(matches!(staging_area, Ok(staging_area) if staging_area.len() == 1))
    }

    #[test]
    pub fn test_02_add_files_to_staging_area() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files");

        let staging_area = version_control_system.add(path);
        assert!(matches!(staging_area, Ok(staging_area) if staging_area.len() == 4))
    }

    #[test]
    pub fn test_03_add_file_1_the_status_is_created() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files/file1.txt");

        let staging_area = version_control_system.add(path);
        assert!(
            matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), path, "CREATED"))
        );
    }

    #[test]
    pub fn test_04_add_file_2_the_status_is_created() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files");
        let file = path.join("file2.txt");

        let staging_area = version_control_system.add(path);
        assert!(
            matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), &file, "CREATED"))
        );
    }

    #[test]
    pub fn test_05_add_file_1_the_status_is_modified() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files/file1.txt");
        version_control_system
            .local_repository
            .insert(path.display().to_string(), "content".to_string());

        let staging_area = version_control_system.add(path);

        assert!(
            matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), &path, "MODIFIED"))
        );
    }

    #[test]
    pub fn test_06_if_file_1_is_in_staging_area_and_add_file_1_updated_staging_area() {
        let mut version_control_system =
            VersionControlSystem::init("tests/utils/files".to_string());
        let path = Path::new("tests/utils/files/file1.txt");

        let _ = version_control_system.add(path);
        let staging_area = version_control_system.add(path);

        assert!(
            matches!(staging_area, Ok(staging_area) if equals(staging_area.clone(), &path, "CREATED"))
        );
    }

    // Tenemos que testear combinaciones entre rm y add

    // IMPORTANTE: Por lo que lei (CHEQUEAR), si deseamos eliminar un archivo y subir los cambios al area intermedia no se usa add, se utiliza rm

    // Algunos test que deberiamos implementar

    // El comando rm lo que hace es eliminar el archivo del directorio actual (tiene que estar si o si commiteado )
    // y subir ese cambio al area de staging
    
        // si archivo1 no esta commiteado => git rm archivo1.txt => rompe
        // si archivo1 esta commiteado => git rm archivo1.txt => 
                                                                // elimina el archivo1.txt del directorio actual
                                                                // agrega al staging area DELETED: archivo1.txt
    // algunos tests para hacer
        // si el archivo1 no esta commiteado, cuando hago rm tira error
        // si el archivo1 esta commiteado, cuando hago rm
                            // staging area tiene DELETED: archivo1.txt
                            // en staging area hay 1 archivo
        // ahora bien, si archivo1 esta en el area de staging => cuando hago git rm archivo1.txt tira error => dice que uses --cached

    // primero ver de implementar rm basico y luego rm --cached 

}

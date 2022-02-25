use specdiff::*;

mod common;


#[test]
fn test_toml_should_work() {
    let toml_str = r#"
    path = "."

    [[addresses]]
    name = "fpaste_f34"
    x = "https://src.fedoraproject.org/rpms/fpaste/raw/rawhide/f/fpaste.spec"
    y = "https://src.fedoraproject.org/rpms/fpaste/raw/f34/f/fpaste.spec"
    
    [[addresses]]
    name = "fpaste_epel8"
    x = "https://src.fedoraproject.org/rpms/fpaste/raw/rawhide/f/fpaste.spec"
    y = "https://src.fedoraproject.org/rpms/fpaste/raw/epel8/f/fpaste.spec"
    "#;

    let result = common::get_address_list(toml_str).unwrap();
    let expected = vec![Address{name : "fpaste_f34".to_string(), x : "https://src.fedoraproject.org/rpms/fpaste/raw/rawhide/f/fpaste.spec".to_string(), y : "https://src.fedoraproject.org/rpms/fpaste/raw/f34/f/fpaste.spec".to_string()},
                                    Address{name : "fpaste_epel8".to_string(), x :"https://src.fedoraproject.org/rpms/fpaste/raw/rawhide/f/fpaste.spec".to_string(), y : "https://src.fedoraproject.org/rpms/fpaste/raw/epel8/f/fpaste.spec".to_string()}];
    assert_eq!(result, expected);
}

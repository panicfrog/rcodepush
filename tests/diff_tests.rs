use rcodepush::prelude::*;

#[test]
fn test_calculate_binary_diff() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![1, 2, 3, 4, 6];
    let ops = calculate_binary_diff(&v1, &v2);
    assert_eq!(ops.len(), 1);
    assert_eq!(
        ops[0],
        Patch::Replace {
            old_index: 4,
            new_index: 4,
            old_value: vec![5],
            new_value: vec![6]
        }
    );
}

#[test]
fn test_calculate_file_hash() {
    match calculate_file_hash("./tests/choose_new_idcard.webp") {
        Ok(h) => assert_eq!("19127e790ea4e3ea", h),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn test_create_directory_blob_file_rec() {
    match create_directory_blob_file_rec("./tests/assets_blobs_rec", "./tests/assets") {
        Ok(h) => assert_eq!("2538abd6c50fd924", h),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn test_create_directory_blob_file() {
    match create_directory_blob_file("./tests/assets_blobs2", "./tests/assets") {
        Ok(h) => assert_eq!("2538abd6c50fd924", h),
        // Ok(h) => println!("{}", h),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn test_create_directory_blob_file2() {
    match create_directory_blob_file("./tests/assets_blobs2", "./tests/assets2") {
        Ok(h) => assert_eq!("8b15ce070738c8ef", h),
        // Ok(h) => println!("{}", h),
        Err(e) => panic!("{:?}", e),
    }
}

#[test]
fn test_compare_blob_files() {
    match compare_blob_files("2538abd6c50fd924", "8b15ce070738c8ef", "./tests/assets_blobs2") {
        Ok(h) => println!("{:?}", h),
        // Ok(h) => println!("{}", h),
        Err(e) => panic!("{:?}", e),
    }
}

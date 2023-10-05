fn main() {
    let data = [
        "<--",
        // "<---", test
        "#####",
        "<==",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>();

    println!("{:?}", data);
    println!("{:?}", vflip(&data));
    println!("{:?}", hflip(&data));
    println!("{:?}", vcat(&data, &data[..2]));
    println!("{:?}", hcat(&data, &data[..2]));
    println!("{:?}", hcat(&data[..2], &data));
}

#[allow(dead_code)]
fn vflip(img: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    for string in img.iter().rev() {
        result.push(string.to_string());
    }
    result
}

#[allow(dead_code)]
fn hflip(img: &[String]) -> Vec<String> {
    let mut final_result = Vec::new();
    let mut max = 0;
    
    for string in img.iter(){
        if string.len() > max {
            max = string.len();
        }
    }

    for string in img.iter() {
        let mut count = 0;
        let mut result = String::new();
        for char in string.chars().rev() {
            result.push(char);
            count += 1;
        }
        while count < max {
            result.insert(0, ' ');
            count += 1;
        }
        
        final_result.push(result.clone())
    }
    final_result
}

#[test]
fn test_img_flip() {
    let emp = ["".to_string(); 0]; 
    assert_eq!(vflip(&emp), [""; 0]); 
    assert_eq!(hflip(&emp), [""; 0]);

    let data = 
    [ "<--",
    "#####",
    "<=="
    ].map(|v| v.to_string());

    assert_eq!(vflip(&data), ["<==", "#####", "<--"]); 
    assert_eq!(hflip(&data), [ "  --<","#####","  ==<"]); 

}

#[allow(dead_code)]
fn vcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();

    for string1 in img1.iter() {
        result.push(string1.to_string());
    }

    for string2 in img2.iter() {
        result.push(string2.to_string());
    }
    result
   
}

fn hcat(img1: &[String], img2: &[String]) -> Vec<String> {
    let mut result = Vec::new();
    let mut max_img1 = 0;

    let mut list_string1 = Vec::new();
    let mut list_string2 = Vec::new();

    for string in img1 {
        list_string1.push(string.to_string());
        if string.len() > max_img1 {
            max_img1 = string.len();
        }
    }

    while list_string1.len() < img2.len() {
        list_string1.push("".to_string());
    }

    for string in img2 {
        list_string2.push(string.to_string());
    }

    while list_string2.len() < img1.len() {
        list_string2.push("".to_string());
    }

    for (index, (string1, string2)) in list_string1.iter().zip(list_string2.iter()).enumerate() {
        let mut formatted_string = string1.to_string();

        while formatted_string.len() < max_img1 {
            formatted_string.push(' ');
        }

        formatted_string.push_str(string2);

        if index == list_string1.len() - 1 {
            result.push(formatted_string.trim_end().to_string());
        } else {
            result.push(formatted_string.to_string());
        }
    }
    result
}

#[test]
fn test_img_cat() {
    let emp = ["".to_string(); 0];
    assert_eq!(vcat(&emp, &emp), [""; 0]);
    assert_eq!(hcat(&emp, &emp), [""; 0]);

    let data = [
        "<--",
        "#####",
        "<==",
    ].map(|v|v.to_string());

    assert_eq!(vcat(&emp, &data), data);
    assert_eq!(vcat(&data, &emp), data);

    assert_eq!(vcat(&data, &data), [
        "<--",
        "#####",
        "<==",
        "<--",
        "#####",
        "<==",
    ]);

    assert_eq!(hcat(&data, &data[..2]), [
        "<--  <--",
        "##########",
        "<=="
    ]);

    assert_eq!(hcat(&data[..2], &data), [
        "<--  <--",
        "##########",
        "     <=="
    ]);

}
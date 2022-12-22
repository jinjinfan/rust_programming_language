use std::collections::HashMap;

fn caculate() {
    let mut vc =vec![1,2,4,4,6,10,12,45];
    {
        let mut sum = 0;
        for num in &vc {
            sum += num;
        }
        let average = sum as f32 / vc.len() as f32;
        println!("average is {}", average);
    }
    {
        vc.sort();
        println!("new vec {:?}", vc);
        let mean = vc[vc.len()/2];
        println!("Mean of vec is: {}", mean);
    }
    let mut hm = HashMap::new();
    {
        for num in &vc {
            let count = hm.entry(num).or_insert(0);
            *count += 1;
        }
    }
    println!("hashmap {:?}", hm);
    let mut max = 0;
    let mut mode = 0;
    for (key, value) in &hm {
        if *value > max {
            mode = **key;
            max = *value;
        }
    }
    println!("mode is {}", mode);
}
const Consonant_Prefix :&str = "ay";
const Vowel_Prefix : &str = "hay";

fn pig_latin(origin : &str) {
    println!("Original string: {}", origin);
    let mut chars = origin.chars().peekable();
    let mut new_s = String::new();
    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                new_s.push(c);
                String::from("-hay")
            },
            'a'..='z' | 'A'..='Z' => {
                format!("-{}ay",c)
            },
            _ => {
                new_s.push(c);
                continue;
            }
        };
        
        while let Some(&c) = chars.peek() {
            match c {
                'a'..= 'z' | 'A'..='Z' => {
                    chars.next();
                    new_s.push(c);
                },
                _ => break,   
            }
        }
        new_s += &suffix;
    }
    println!("Pig latin new string: {}", new_s);
}

fn company_directory(company : &mut HashMap<String, Vec<String>>, text : &str) {
    let v : Vec<&str> = text.split(' ').collect();
    println!("Splitted words: {:?}", v);
    if v.len() != 4 {
        panic!("Input text should have 4 words!")
    }
    else if v[0] != "Add" && v[2] != "to" {
        panic!("First word should be Add, and third word should be to !")
    }

    let name_list = company.entry(String::from(v[3])).or_insert(Vec::new());
    name_list.push(String::from(v[1]));
}


fn main() {
    println!("Caculate average, mean, mold of integer vector!");
    caculate();
    println!("Pig latin!");
    pig_latin(&String::from("first"));
    pig_latin(&String::from("apple"));
    println!("Company_repository!");
    let mut directory : HashMap<String, Vec<String>> = HashMap::new();
    company_directory(&mut directory, &String::from("Add Sally to Engineering"));
    company_directory(&mut directory, &String::from("Add July to Engineering"));
    company_directory(&mut directory, &String::from("Add Adele to Engineering"));
    company_directory(&mut directory, &String::from("Add Eric to Sales"));
    company_directory(&mut directory, &String::from("Add Simon to Sales"));
    company_directory(&mut directory, &String::from("Add Q to IT"));
    println!("Compaies directory: {:?}", directory);
    for (department, name_list) in directory {
        println!("Department: {}", department);
        let mut names = name_list.clone();
        names.sort();
        println!("\tnames: {:?}", names);

    }


    

    

}

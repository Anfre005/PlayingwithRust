pub fn inspect(st : &String){
    if st.ends_with('s'){
        println!("The contents of the string is Plural");
    }
    else{
        println!("The contents of the string is Singular")
    }
}

pub fn change(st : &mut String){
    if st.ends_with('s'){
        
    }
    else{
        st.push_str("s");
    }
}

pub fn eat(st: String)->bool{
    if st.starts_with('b') && st.contains('a'){
        true
    }
    else{
        false
    }
}

pub fn bedazzle(st: &mut String){
    *st = "sparkly".to_string();
}
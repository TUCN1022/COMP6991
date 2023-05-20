
use std::{error::Error,collections::HashMap};
use csv::{ReaderBuilder};
use regex::Regex;

const ENROLMENTS_PATH: &str = "enrolments.psv";
#[allow(dead_code)]
#[warn(unused_variables)]
struct Data {
    course_code: String,
    student_num: String,
    name: String,
    program: String,
    plan: String,
    wam: f64,
    session: String,
    birthdate: String,
    sex: String,
}

fn main()-> Result<(), Box<dyn Error>>  {
    // example();
    let mut rdr = ReaderBuilder::new().has_headers(false).delimiter(b'|').from_path(ENROLMENTS_PATH)?;
    let mut datas:Vec<Data>=Vec::new();
    
    //cut the data into Vec
    for result in rdr.records() {
        let linedata = result?;
        // println!("{:?}", linedata);
        let course_code=linedata[0].trim().to_string();
        let student_num=linedata[1].to_string();
        let name=linedata[2].trim().to_string();
        let program=linedata[3].to_string();
        let plan=linedata[4].trim().to_string();
        let wam:f64 =linedata[5].parse().unwrap();
        let session=linedata[6].trim().to_string();
        let birthdate=linedata[7].to_string();
        let sex=linedata[8].trim().to_string();
        let data=Data{course_code,student_num,name,program,plan,wam,session,birthdate,sex};
        datas.push(data);
    }
    //find the repeat
    let mut student_map=HashMap::new();
    let mut course_map=HashMap::new();
    let mut cse_student_map=HashMap::new();
    let mut cse_wam_vec:Vec<f64>=Vec::new();
    // let re_pattern=Regex::new(r"^COMP.*|.*\sCOMP.*").unwrap();
    // let re_pattern=Regex::new(r"^COMP").unwrap();


    for student in datas.iter(){
        let student_repeation=student_map.entry(& student.student_num).or_insert(0);
        *student_repeation+=1;
        let course_repeation=course_map.entry(& student.course_code).or_insert(0);
        *course_repeation+=1;

        // if re_pattern.is_match(&student.plan){
        let cse_student_repeation=cse_student_map.entry(& student.student_num).or_insert(0);
        match cse_student_repeation{
            0 =>{
                cse_wam_vec.push(student.wam);
                *cse_student_repeation+=1;
                // println!("{} {} {}",student.student_num,student.plan,student.wam);
            },
            _ => {
                
            },
        }
        // }else{
        //     // println!("{}",student.plan);
        // }
        
    }
    
    let max_course_iter=course_map.iter().max_by_key(|&(_,v)| v).unwrap();
    let min_course_iter=course_map.iter().min_by_key(|&(_,v)| v).unwrap();
    let cse_total_wam:f64=cse_wam_vec.iter().sum();


    println!("Number of students: {}",student_map.len());
    println!("Most common course: {} with {} students",max_course_iter.0,max_course_iter.1);
    println!("Least common course: {} with {} students",min_course_iter.0,min_course_iter.1);

    // println!("{}",cse_total_wam);
    // println!("{}",cse_wam_vec.len() as f64);
    // println!("Average WAM: {:}",cse_total_wam/cse_wam_vec.len() as f64);
    println!("Average WAM: {:.2}",cse_total_wam/cse_wam_vec.len() as f64);
    
    
    
    Ok(())
}

use std::io;

fn main() {
    println!("Please input 'kasi' or 'sessi'!");
    let mut si = String::new();
    io::stdin()
        .read_line(&mut si)
        .expect("failed to read line!");

    si = si.trim_end().to_string();
    let si = si.as_str();

    if si != "sessi" && si != "kasi"{
        eprintln!("Error,try again!");
        return;
    } 

    println!("Please input temperature!");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("faile to read line!");
    temp = temp.trim_end().to_string();

    let temp: f64 = temp.parse().expect("faile!");

    let mut res = 0.0;

    if si == "kasi"{
        res = kasi_to_sessi(temp);
    }else if si == "sessi"{
        res = sessi_to_kasi(temp);
    }

    println!("tempreture:{} ",res);

}



fn kasi_to_sessi(f: f64) -> f64{
    let se;
    se = (f - 32.0) / 1.8;

    se
}

fn sessi_to_kasi(f: f64) -> f64{
    let ka;
    ka = (f * 1.8) + 32.0;

    ka
}
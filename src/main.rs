use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::{Ipv4Addr};
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use ipnet::{Ipv4Net};

fn main() {
    let args: Vec<String> = env::args().collect();

    let out_mode :(bool, Option<String>) =
        if args.contains(&String::from("-o"))
            {(true, None)}
        else
            {(false, None)};


    if args.len() < 2 {
        println!("Ip cidr utiliy written in rust\n");
        println!("USAGE:");
        println!("\tcidrutil [OPTIONS]\n");
        println!("OPTIONS:");
        println!("\t-c, --contains\t<IPFILE>  <SUBFILE>\tChecks if the list of ip in input is contained in the list of subnets");
        println!("\t-e, --explode\t<SUBFILE>\t\tExplodes the subnet addresses in input");
        println!("\t-o, --output\t<OUTFILE>\t\tSave the output in a csv file")
    } else if args.contains(&String::from("-c"))
        || args.contains(&String::from("--contains")) {
        let ipfile = match args.get(2) {
            None => {
                eprintln!("Invalid number of argument passed");
                exit(1);
            },
            Some(ip) => ip,
        };
        let subfile = match args.get(3) {
            None => {
                eprintln!("Invalid number of argument passed");
                exit(1);
            },
            Some(sub) => sub,
        };

        cidr_contain(ipfile, subfile, out_mode);
    }
}

fn cidr_contain (ipfile : &String, subfile : &String, output :(bool, Option<String>)) {
    let mut iplist :Vec<Ipv4Addr> = Vec::new();
    let mut sublist :Vec<Ipv4Net> = Vec::new();
    let mut csublist :HashMap<Ipv4Net, i8> = HashMap::new();

    if let Ok(lines) = read_lines(ipfile) {
        for line in lines {
            if let Ok(ip) = line {
                match Ipv4Addr::from_str(&ip) {
                    Ok(address) => iplist.push(address),
                    _ => {
                        eprintln!("[Warning] Skipped unparsable address, {}", ip);
                    }
                };
            }
        }
    }

    if let Ok(lines) = read_lines(subfile) {
        for line in lines {
            if let Ok(sub) = line {
                match Ipv4Net::from_str(&sub) {
                    Ok(address) => sublist.push(address),
                    _ => {
                        eprintln!("[Warning] Skipped unparsable subnet: {}", sub);
                    }
                };
            }
        }
    }

    for ip in &iplist {
        for sub in &sublist {
            if sub.contains(ip) {
                if !csublist.contains_key(sub) {
                    csublist.insert(*sub, 1);
                } else {
                    *csublist.get_mut(sub).unwrap() += 1;
                }
            }
        }
    }

    let mut sub_vec :Vec<_> = csublist.iter().collect();
    sub_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("IP ADDR\t\t CIDR");
    for &x in &sub_vec {
        println!("{}\t {}", x.0, x.1);
    }

    // Writing the results in csv if we have the user want
    if output.0 {
        let outname :String = output.1.unwrap_or(String::from("sub-out.txt"));
        let mut wrt = csv::Writer::from_path(&outname).unwrap();

        wrt.write_record(["IP CIDR", "COUNT"]).unwrap();

        for &x in &sub_vec {
            wrt.write_record(&[x.0.to_string(), x.1.to_string()]).unwrap();
        }
        wrt.flush().unwrap();

        println!("Result writter to file {}", &outname);
    }

}

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
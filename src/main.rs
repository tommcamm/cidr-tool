use std::collections::HashMap;
use std::fs::{File};
use std::io::{self, BufRead};
use std::net::{Ipv4Addr};
use std::ops::Add;
use std::path::{Path};
use std::str::FromStr;
use clap::{arg, command, Command};
use ipnet::{Ipv4Net};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let matches = command!()
        .about("CLI tool written in rust for helping tasks related to ipv4 CIDR's")
        .arg_required_else_help(true)
        .arg(arg!(
            -d --debug ... "Turns on debug mode"
        ))
        .subcommand(
            Command::new("contains")
                .about("checks from a given subnet list, how many contains the given ip's")
                .arg(
                    arg!(-i --ipfile <FILE> "Path to the ip file")
                        .required(true),
                )
                .arg(
                    arg!(-s --subfile <FILE> "Path to the subnet file")
                        .required(true),
                ).arg(
                    arg!(-o --outfile <FILE>
                        "Outputs the result in the specified file")
                        .required(false)),
        )
        .subcommand(
            Command::new("explode")
                .about("Explode the subnets given in input in a csv format")
                .arg(
                arg!(-s --subfile <FILE> "Path to the subnet file")
                    .required(true),
                ).arg(
                arg!(-o --outfile <FILE>
                        "Path for the output file [default is ./cidr-out.csv]")
                    .required(false)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("contains") {
        let ipfile = matches.value_of("ipfile").unwrap();
        let subfile = matches.value_of("subfile").unwrap();

        let outfile :Option<String> = match matches.value_of("outfile") {
            Some(inp) => Option::from(String::from(inp)),
            None => None,
        };

        cidr_contain(&String::from(ipfile), &String::from(subfile),
                     outfile);
    } else if let Some(matches) = matches.subcommand_matches("explode") {

        let subfile = matches.value_of("subfile").unwrap();
        let outfile :String = match matches.value_of("outfile") {
            Some(inp) => String::from(inp),
            None => String::from("cidr-out.csv"),
        };

        subnets_exploder(String::from(subfile), outfile);
    } else {
        println!()
    }
}

fn cidr_contain (ipfile : &String, subfile : &String, output :Option<String>) {
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

    println!("IP ADDR\t\t COUNT");
    for &x in &sub_vec {
        println!("{}\t {}", x.0, x.1);
    }

    // Writing the results in csv if we have the user want
    if output.is_some() {
        let outname :String = output.unwrap();
        let mut wrt = csv::Writer::from_path(&outname).unwrap();

        wrt.write_record(["IP CIDR", "COUNT"]).unwrap();

        for &x in &sub_vec {
            wrt.write_record(&[x.0.to_string(), x.1.to_string()]).unwrap();
        }
        wrt.flush().unwrap();

        println!("\n[Info] Result written to file {}", &outname);
    }

}

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn subnets_exploder(subfile :String, outfile :String) {
    let mut sublist :Vec<Ipv4Net> = Vec::new();

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

    let mut count :i8 = 0;
    let mut handle_vec :Vec<JoinHandle<()>> = Vec::new();

    for x in sublist {
        handle_vec.push(thread::spawn(move || {
            let fname :String = String::from("/tmp/cidrtmp").add(&count.to_string());
            subnet_explode(x, String::from(fname));
        }));
        count += 1;
    }

    let mut opfile = csv::Writer::from_path(&outfile).unwrap();

    //wait for all threads
    let mut n :i8 = 0;
    for x in handle_vec {
        x.join().unwrap();
        if let Ok(lines) = read_lines(String::from("/tmp/cidrtmp")
            .add(&n.to_string())) {
            for line in lines {
                if let Ok(ip) = line {
                    opfile.write_record(&[ip.to_string()]).unwrap();
                }
            }
        }
        n += 1;
    }

    opfile.flush().unwrap();
    println!("subnet explosion finished, results written to {}", outfile);
}

fn subnet_explode(net :Ipv4Net, filename :String) {
   let mut wrt = csv::Writer::from_path(&filename).unwrap();
    for x in net.hosts() {
        wrt.write_record(&[x.to_string()]).unwrap();
    }

    wrt.flush().unwrap();
}
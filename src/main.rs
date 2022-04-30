use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::{Ipv4Addr};
use std::ops::Add;
use std::path::{Path};
use std::str::FromStr;
use clap::{arg, command, Command};
use ipnet::{Ipv4Net};

fn main() {
    let matches = command!()
        .arg(arg!(
            -v --verbose ... "Turns on verbose mode"
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


        // foo
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
        let net: Ipv4Net = "52.96.0.0/12".parse().unwrap();
        subnets_exploder(String::from("sub-list.txt"));
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

    println!("IP ADDR\t\t CIDR");
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

        println!("Result written to file {}", &outname);
    }

}

fn read_lines<P> (filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn subnets_exploder(subfile :String) {
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
    for x in sublist {
        let fname :String = String::from("tmpip").add(&count.to_string());
        subnet_explode(x, String::from(fname));
        count += 1;
    }


}

fn subnet_explode(net :Ipv4Net, filename :String) {
   let mut wrt = csv::Writer::from_path(&filename).unwrap();
    for x in net.hosts() {
        wrt.write_record(&[x.to_string()]).unwrap();
    }

    wrt.flush().unwrap();
}
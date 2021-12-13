use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Network {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

impl FromStr for Network {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s
            .clone()
            .lines()
            .map(|x| x.split_once("-").unwrap())
            .map(|(n, e)| (n.trim().to_string(), e.trim().to_string()))
            .fold(
                HashMap::new(),
                |mut acc: HashMap<String, HashSet<String>>, (n, e)| {
                    if !acc.contains_key(&n) {
                        acc.entry(n.clone()).or_insert(HashSet::new());
                    }
                    if e != "start" {
                        acc.entry(n.clone()).and_modify(|x| {
                            x.insert(e.clone());
                        });
                    }

                    if !acc.contains_key(&e) {
                        acc.entry(e.clone()).or_insert(HashSet::new());
                    }
                    if n != "start" {
                        acc.entry(e).and_modify(|x| {
                            x.insert(n);
                        });
                    }

                    acc
                },
            );
        //.collect::<HashMap<String, String>>();

        let nodes = edges.iter().fold(HashSet::new(), |mut acc, (n, e)| {
            acc.insert(n.to_string());
            let un: HashSet<String> = acc.union(e).map(|x| x.to_string()).collect();
            return un;
        });

        Ok(Network {
            nodes: nodes,
            edges: edges,
        })
    }
}

pub fn find_paths_1(network: Network, used: HashSet<String>, node: String) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];
    for option in network.edges[&node].clone() {
        if option == "end" {
            let mut ending = vec![vec!["end".to_string()]];
            paths.append(&mut ending);
        } else if option.to_uppercase() == option {
            let mut results = find_paths_1(network.clone(), used.clone(), option.clone())
                .iter()
                .map(|vs| {
                    let mut vsc = vs.clone();
                    vsc.push(option.clone());
                    return vsc.clone();
                })
                .collect::<Vec<Vec<String>>>();

            paths.append(&mut results);
        } else if !used.contains(&option) {
            let mut next = used.clone();
            next.insert(option.clone());
            let mut results = find_paths_1(network.clone(), next, option.clone())
                .iter()
                .map(|vs| {
                    let mut vsc = vs.clone();
                    vsc.push(option.clone());
                    return vsc.clone();
                })
                .collect::<Vec<Vec<String>>>();
            paths.append(&mut results);
        }
    }
    return paths;
}

pub fn go_1(network: Network) -> Vec<Vec<String>> {
    return find_paths_1(network, HashSet::new(), "start".to_string());
}

pub fn find_paths_2(network: Network, used: HashSet<String>, node: String, doubled: Option<String>) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = vec![];
    for option in network.edges[&node].clone() {
        if option == "end" {
            let mut ending = vec![vec!["end".to_string()]];
            paths.append(&mut ending);
        } else if option.to_uppercase() == option {
            let mut results = find_paths_2(network.clone(), used.clone(), option.clone(), doubled.clone())
                .iter()
                .map(|vs| {
                    let mut vsc = vs.clone();
                    vsc.push(option.clone());
                    return vsc.clone();
                })
                .collect::<Vec<Vec<String>>>();

            paths.append(&mut results);
        } else if !used.contains(&option) {
            let mut next = used.clone();
            next.insert(option.clone());
            let mut results = find_paths_2(network.clone(), next, option.clone(), doubled.clone())
                .iter()
                .map(|vs| {
                    let mut vsc = vs.clone();
                    vsc.push(option.clone());
                    return vsc.clone();
                })
                .collect::<Vec<Vec<String>>>();
            paths.append(&mut results);
        }
        else if doubled == None {
            let next = Some(option.clone());
            let mut results = find_paths_2(network.clone(), used.clone(), option.clone(), next)
                .iter()
                .map(|vs| {
                    let mut vsc = vs.clone();
                    vsc.push(option.clone());
                    return vsc.clone();
                })
                .collect::<Vec<Vec<String>>>();
            paths.append(&mut results);
        }
    }
    return paths;
}

pub fn go_2(network: Network) -> Vec<Vec<String>> {
    return find_paths_2(network, HashSet::new(), "start".to_string(), None);
}
pub fn main() {
    let s = "start-end";
    let t = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
    let r = "dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";
    let q = "fs-end
    he-DX
    fs-he
    start-DX
    pj-DX
    end-zg
    zg-sl
    zg-pj
    pj-he
    RW-he
    fs-DX
    pj-RW
    zg-RW
    start-pj
    he-WI
    zg-he
    pj-fs
    start-RW";
    let z = "hl-WP
    vl-fo
    vl-WW
    WP-start
    vl-QW
    fo-wy
    WW-dz
    dz-hl
    fo-end
    VH-fo
    ps-vl
    FN-dz
    WP-ps
    ps-start
    WW-hl
    end-QW
    start-vl
    WP-fo
    end-FN
    hl-QW
    WP-dz
    QW-fo
    QW-dz
    ps-dz";

    let network_s = Network::from_str(s).unwrap();
    let network_t = Network::from_str(t).unwrap();
    let network_r = Network::from_str(r).unwrap();
    let network_q = Network::from_str(q).unwrap();
    let network_z = Network::from_str(z).unwrap();

    for (i, path) in go_1(network_s.clone()).iter().enumerate() {
        print!("{}: start",i+1);
        for p in path.iter().rev() {
            print!(",{}", p);
        }
        println!("");
    }

    assert_eq!(go_1(network_s.clone()).len(),1);
    assert_eq!(go_1(network_t.clone()).len(),10);
    assert_eq!(go_1(network_r.clone()).len(),19);
    assert_eq!(go_1(network_q.clone()).len(),226);
    assert_eq!(go_1(network_z.clone()).len(),3410);

    assert_eq!(go_2(network_s).len(),1);
    assert_eq!(go_2(network_t).len(),36);
    assert_eq!(go_2(network_r).len(),103);
    assert_eq!(go_2(network_q).len(),3509);
    assert_eq!(go_2(network_z).len(),98796);
    // s t r q z 
}

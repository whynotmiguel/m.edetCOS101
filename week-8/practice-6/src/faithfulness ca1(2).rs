use std::io::{self, Write};


struct Record {
    sn: usize,
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn main() -> io::Result<()> {
   
    let commissioners = vec![
        "Aigbogun Alamba Daudu".to_string(),
        "Murtala Afeez Bendu".to_string(),
        "Okorocha Calistus Ogbonna".to_string(),
        "Adewale Jimoh Akanbi".to_string(),
        "Osazuwa Faith Etiye".to_string(),
    ];
    
    let ministries = vec![
        "Internal Affairs".to_string(),
        "Justice".to_string(),
        "Defense".to_string(),
        "Power & Steel".to_string(),
        "Petroleum".to_string(),
    ];
    
    let geopolitical_zones = vec![
        "South West".to_string(),
        "North East".to_string(),
        "South South".to_string(),
        "South West".to_string(),
        "South East".to_string(),
    ];

    
    if commissioners.len() != ministries.len() || commissioners.len() != geopolitical_zones.len() {
        println!("Error: Datasets have mismatched lengths.");
        return Ok(());
    }

    
    let mut records = Vec::new();
    for i in 0..commissioners.len() {
        records.push(Record {
            sn: i + 1,
            name: commissioners[i].clone(),
            ministry: ministries[i].clone(),
            geopolitical_zone: geopolitical_zones[i].clone(),
        });
    }

   
    let mut file = File::create("merged_records.txt")?;

    
    writeln!(
        file,
        "{:<5} {:<30} {:<20} {:<15}",
        "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE"
    )?;

    
    for record in records {
        writeln!(
            file,
            "{:<5} {:<30} {:<20} {:<15}",
            record.sn, record.name, record.ministry, record.geopolitical_zone
        )?;
    }

    println!("Data has been successfully written to 'merged_records.txt'.");
    Ok(())
}

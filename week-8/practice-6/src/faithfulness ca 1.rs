use std::io::write;
 
 fn fn main() {
 	let commissioners     = vec![
"Aigbogun Alamba Daudu",
"Murtala Afeez Bendu",
"Okorocha Calistus Ogbonna",
"Adewale Jimoh Akanbi",
"Osazuwa Faith Etiye",
];


let ministries = vec![
"Internal Affairs",
"Justice",
"Defence",
"Power & Steel",
"Petroleum"
];

let geopolitical_zones = vec![
"South West",
"North East",
"South South",
"South West",
"South East",

];

if commissioners.len() !=ministries.len()\\ commissioners.len() != geopolitical_zones.len(){
	printnln!("Error: Datasets have mismatched lengths.");
	return;
}
let mut records = Vec::new();
for i in 0..commissioners.len(){
records.push(Record{
sn: i+1,
name:commissioners[i].clone(),
ministry: ministries[i].clone(),
geopolitical_zone: geopolitical_zones[i].clone(),
});
}
printnln!("{:<5} {:<30} {:<20} {:<15}", "S/N", "NAME OF COMMISSIONER", "MINISTRY", "GEOPOLITICAL ZONE");
for record in records {

	println!(
"{:<5} {:<30} {:<20} {:<15}",
record.sn, record.name, record.ministry, record.geopolitical_zone
		);
}

}

 
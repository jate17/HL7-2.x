use chrono::NaiveDateTime;


const MSG_HL7: &str = "MSH|^~\\&|SendingApp|SendingFac|ReceivingApp|ReceivingFac|202310261200||ADT^A01|MSG00001|P|2.5
PID|1||123456^^^Hospital^MR||Rossi^Mario||19800101|M|||Via Roma 1^^Roma^^00100||3331234567|||M
PV1|1|I|Ward^101^1^Hospital|||Dr. Bianchi^^^MD";



/*
const MSG_HL7: &str = "||||||202310261jhkhj200||ADT1||F|3
PID|1||123456^^^Hospital^MR||Rossi^Mario||19800101|M|||Via Roma 1^^Roma^^00100||3331234567|||M
PV1|1|I|Ward^101^1^Hospital|||Dr. Bianchi^^^MD";
*/

fn validate_hl7_datetime(dt_str: &str) -> bool {
    NaiveDateTime::parse_from_str(dt_str, "%Y%m%d%H%M").is_ok()
}


fn validate_header(header: &Vec<&str>)  -> Vec<String> {
        let mut feedbacks: Vec<String> = Vec::new();

    if header.len() < 12 {
        feedbacks.push(format!("Lunghezza {} campi non rispetta i minimi", header.len()));
    }

    if header[0] != "MSH" {
        feedbacks.push("MSH non presente o incorretto".to_string());
    }

    let len_delimitatori = header[1].len();
    if !(2..=4).contains(&len_delimitatori) {
        feedbacks.push(format!("Lunghezza campo delimitatori {} non corretta", len_delimitatori));
    }

    if header[2].is_empty() {
        feedbacks.push("Applicazione mittente mancante".to_string());
    }

    if header[3].is_empty() {
        feedbacks.push("Struttura o istituto mittente mancante".to_string());
    }

    if header[4].is_empty() {
        feedbacks.push("Applicazione destinataria mancante".to_string());
    }

    if header[5].is_empty() {
        feedbacks.push("Struttura o istituto destinatario mancante".to_string());
    }

    if !validate_hl7_datetime(header[6]) {
        feedbacks.push("Data non valida o mancante".to_string());
    }

    if header[8].is_empty() {
        feedbacks.push("Tipo di evento mancante".to_string());
    } else {
        let parts: Vec<&str> = header[8].split('^').collect();
        if parts.len() != 2 {
            feedbacks.push("Tipo di evento non completo".to_string());
        }
    }

    if header[9].is_empty() {
        feedbacks.push("ID messaggio mancante".to_string());
    }

    if header[10].is_empty() {
        feedbacks.push("Processing ID mancante".to_string());
    } else if !(header[10] == "P" || header[10] == "T" || header[10] == "D") {
        feedbacks.push("Processing ID non conforme".to_string());
    }

    if header[11].is_empty() {
        feedbacks.push("Versione mancante".to_string());
    } else {
        let v = ["2.1", "2.2", "2.3", "2.4", "2.5", "2.6", "2.7", "2.8"];
        if !v.contains(&header[11]) {
            feedbacks.push("Versione non conforme".to_string());
        }
    }

    feedbacks



}

fn main() {

    let segments: Vec<&str> = MSG_HL7.split("\n").collect();

    let header: Vec<&str> = segments[0].split("|").collect();

    let s = validate_header(&header);
    println!("{:?}", s);
}

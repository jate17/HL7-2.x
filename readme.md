



MSH|^~\&|SendingApp|SendingFac|ReceivingApp|ReceivingFac|202310261200||ADT^A01|MSG00001|P|2.5

MSH è il segmento di intestazione del messaggio.
| è il delimitatore dei campi.
^~\& (campo 2) indica i delimitatori di sottocampi e di escape usati nel messaggio.
SendingApp (campo 3) è l'applicazione mittente.
SendingFac (campo 4) è la struttura mittente.
ReceivingApp (campo 5) è l'applicazione destinataria.
ReceivingFac (campo 6) è la struttura destinataria.
202310261200 (campo 7) è la data e ora di invio (formato YYYYMMDDHHMM).
Campo 8 è vuoto (sequenza evento opzionale).
ADT^A01 (campo 9) è il tipo di messaggio, qui un evento di ammissione paziente (ADT) tipo A01 (registrazione).
MSG00001 (campo 10) è un identificativo univoco del messaggio.
P (campo 11) indica la priorità.
2.5 (campo 12) è la versione dello standard HL7 usata.


Segmento PID (Patient Identification)

PID|1||123456^^^Hospital^MR||Rossi^Mario||19800101|M|||Via Roma 1^^Roma^^00100||3331234567|||M
PID identifica il paziente.
1 è il numero del segmento.
Campo 3 contiene l'ID paziente: 123456 con codice localizzatore Hospital e tipo MR (Medical Record Number).
Campo 5 è il nome: cognome Rossi e nome Mario.
Campo 7 è la data di nascita: 1 gennaio 1980.
Campo 8 è il sesso: M maschio.
Campo 11 è l'indirizzo: Via Roma 1, città Roma, CAP 00100.
Campo 13 è il telefono: 3331234567.
Campo 16 è lo stato civile, qui M (maritato).



Segmento PV1 (Patient Visit)

PV1|1|I|Ward^101^1^Hospital|||Dr. Bianchi^^^MD
PV1 contiene info sulla visita o ricovero.
1 è il numero del segmento.
I indica stato "inpatient" (ricoverato).
Il campo 3 indica la stanza o reparto: Ward reparto, stanza 101.
Campo 7 è il medico curante: Dr. Bianchi, tipo professionale MD (Medico).


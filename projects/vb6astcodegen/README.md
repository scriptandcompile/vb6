# VBA6: ricostruzione Rust AST → P-code

Crate Rust senza dipendenze. `emit_ast` consuma un AST già tipizzato, associato ai simboli e abbassato alla rappresentazione usata dal dispatcher originale `FUN_0fabf2d2`. Restituisce lo stream binario, gli opcode logici con operandi e funzione originale, e il valore di ritorno contestuale del dispatcher. Rifiuta esplicitamente i quattro nodi builtin da abbassare e i nodi argomento `0x70/0x71` che richiedono il binder. Non esegue la DLL o lo snapshot per generare il codice.

La ricostruzione è utilizzabile per i percorsi implementati; non è ancora l'intero compilatore VBA6. I risultati dei confronti differenziali dimostrano equivalenza con le istruzioni dello snapshot fornito nei casi esercitati, non equivalenza generale con un compilatore Microsoft in esecuzione.

## Uso

```rust
use vba6_ast_codegen::{emit_ast, Ast, Bindings, Node};

let mut ast = Ast::default();
let a = ast.push(Node::integer(6, 7));
let b = ast.push(Node::integer(6, 2));
let root = ast.push(Node::operator(0x16, 6, a, Some(b)));
let result = emit_ast(&ast, root, 1, &Bindings::default())?;
assert_eq!(result.bytes, [0xF4, 7, 0xF4, 2, 0xA9]);
# Ok::<(), vba6_ast_codegen::EmitError>(())
```

```sh
cargo test
cargo run --example arithmetic
cargo clippy --all-targets -- -D warnings
```

`type_id=6` è il tipo nativo di questo esempio: i valori non sono un enum Rust nuovo. `mode` conserva il parametro contestuale originale. ID AST, tipo, flag, figli e operandi partecipano insieme alla selezione: il solo ID AST non determina un opcode unico.

`Ast` contiene nodi in arena; `left/right` sono indici, non indirizzi del processo originale. Le liste argomenti conservano il tag `0x37` e i collegamenti originali. La funzione gestisce l'ordine di valutazione di ciascun ramo. I campi `symbol`, `reference` e `member` sono handle normalizzati: il significato del campo nativo corrispondente dipende dal tag. In particolare, per `0x61`, `left` rappresenta il target nativo `node[6]`, `symbol` il simbolo in `node[8]`, `count` la word in `node[7]`.

`Bindings` deve contenere i risultati dei helper ancora esterni: slot di storage (`FAAB81F`), indici del pool (`FAC2E29`), descrittori, classi e informazioni dei simboli. Non vengono assegnati slot o riferimenti fittizi. Per le chiamate `0x61`, `argument_bytes` viene calcolato dall’AST con la formula di `FAC38D1`; può essere fornito per rappresentare un risultato già risolto. Descrittori e metadati mancanti causano un errore. Per `0x60`, `native_references` fornisce i record e i risultati dei helper: `FAC3B64`, `FAC0FA8`, `FAC41E0` e `FB8E726` sono ricostruiti e il codice genera anche i prefissi. `storage_references` offre il percorso compatto per i record ordinari delle classi1..3. Il vecchio `prepared_addresses` resta limitato alla preparazione senza prefissi. `signature_descriptors` rappresenta esito o errore del percorso speciale `FAC35B5`.

L'emissione è transazionale: in caso di errore non restituisce uno stream parziale. Le mutazioni originali necessarie ai rami supportati sono applicate a una copia interna dell'AST. Cicli, profondità eccessiva, riferimenti mancanti e superamento della dimensione dello stream producono errori espliciti.

Per un AST che contiene i quattro nodi runtime recuperati, usare:

```rust,ignore
let (emission, lowering_trace) = emit_ast_with_builtin_lowering(
    &ast, root, mode, &bindings, &builtin_bindings,
)?;
```

`BuiltinBindings` associa gli ID builtin osservati9/12/0x9B/0x140 ai simboli e alle firme risolti nel contesto reale. Il modulo trasforma0x3C/3D/40/5B in chiamate0x61, conserva gli argomenti, materializza lo zeroI4 richiesto dalla distruzione array e prepara il valore di cleanup GoSub con il wrapper nativo tipo23. La verifica dei record originali mostra type-template2 per tutte le quattro firme. Questa funzione non esegue l'intero binder/ottimizzatore `FAEC8D3`; se restano nodi `0x70/0x71`, la generazione fallisce indicando la funzione nativa di binding invece di omettere l'argomento.

Il percorso di riferimenti `native_references` richiede `ResolvedReference`: record simbolo, esiti dei helper, word del contesto e i20 word originali del descrittore selezionabile per classe4. Questi20 word rappresentano gli indirizzi consultabili con gli indici di tipo recuperati, non la dimensione dimostrata di un record. Le modalità, la composizione, gli opcode e i prefissi sono calcolati dalle funzioni Rust. Errori nativi dei lookup remoti diventano errori espliciti; il chiamante deve fornire un risultato reale del helper, senza inventare un indice.

## Tabelle per il codice

| Modulo | Contenuto e provenienza |
|---|---|
| `emission_rules.rs` | 116 route numeriche, basi e flag originali, routine di selezione; tabella usata dal dispatcher Rust |
| `lowering.rs` | `lower_known_builtins` e `emit_ast_with_builtin_lowering` per0x3C/3D/40/5B, con simboli runtime risolti |
| `references.rs` | Preparazione dei riferimenti, classi1..15 e fallback originale, prefissi di ambiente/oggetto, composizione e lookup remoti |
| `ast_catalog.rs` | 116 ID `0x00..0x73`, nomi proposti, significato, stato semantico e riferimenti alle righe delle funzioni decompilate |
| `lowering_hints.rs` | Quattro associazioni osservate AST → builtin runtime, con indirizzi e nomi originali; non sono opcode da emettere direttamente |
| `tables.rs` | Basi AST, flag, indici dei tipi, record opcode, codec binario, basi conversione, dimensioni degli argomenti e lookup di indirizzamento |
| `conversions.rs` | Selezione `FAC0C65` e ottimizzazione costanti `FAC31C9`, con funzione e riga sorgente nei piani |
| `addressing.rs` | Selezione `FAC13F1`, descrittori1..12 e zero/fallback, operandi e indirizzi dei blocchi originali |
| `assignment.rs` | Piano `FAC0827` e switch non-Variant `0x2D`; risultati dei helper richiesti esplicitamente |
| `member_calls.rs` | Selezione e trasformazioni `FAC3506`, `FAC38AE`, `FB91384`, code degli operandi di `0x61` |
| `cleanup.rs` | Lista di cleanup e piano `FB0B52F` |
| `statement_catalog.rs` | Namespace separato dei 26 tag statement, formule e limiti; non è un emettitore di statement completo |
| `emitter.rs` | Traversal, valutazione, selezione, operandi e concatenazione dello stream |

Stati semantici del catalogo: **10 OBSERVED, 99 INFERRED, 7 UNKNOWN**. La nuova classificazione di `0x35` è una descrizione strutturale inferita, non un significato VBA dimostrato. Questi stati riguardano i nomi e i significati; un comportamento di emissione può essere osservato anche quando il nome del nodo resta sconosciuto.

Le formule e i valori numerici derivano dalle sorgenti decompilate e dal VBALIFT1 incluso nei Rust forniti. `OPCODE-MATRIX.csv` aiuta ad attribuire nomi agli opcode, non sostituisce l'evidenza delle formule. I nomi proposti non sono nomi di un enum Microsoft recuperato.

| Tabella | Indirizzo nativo | Estensione usata |
|---|---|---:|
| `OPCODE_BASE` | `0x0FAC2BA8` | 115 word |
| `AST_FLAGS` | `0x0FAA6C98` | 115 byte |
| `TYPE_INDEX` | `0x0FAAA670` | 30 dword originali, valori conservati come u16 |
| `CONVERSION_BASE` | `0x0FAC2940` | **12** dword originali |
| `OPCODE_RECORDS` | `0x0FAC2090` | 1095 record di 2 byte |
| `CANONICAL_SHORT` | `0x0FB09508` | 251 word |
| `ARGUMENT_BYTES` | `0x0FAC39C0` | 30 word |
| `ADDRESS_LOOKUP_BYTES` | `0x0FAC29B0` | 144 byte, accesso tramite formula `FAC16ED..FAC1703` |
| `GET_PUT_BASE` | switch `0x0FAC41C8` | 6 casi: `FAC415B`, `FB35812`, `FAC4187`, `FB35824`, `FB3583B`, `FB3584D` |

Correzione rispetto alla precedente ricerca: il dump `conversion_bases.bin` contiene 20 dword, ma solo i primi 12 sono basi di conversione. Da `0x0FAC2970` iniziano indirizzi di switch. Il codice non usa questi indirizzi come opcode. Le estensioni indicate sono quelle recuperate e usate, non una prova dei confini globali di tutte le strutture.

Snapshot sorgente VBALIFT1, SHA256: `0634ed366702d7c4ee3fbd09c4969ca9cc82213c5a1a45e0cdc065316a1c75a3`. I digest dei principali dump sono nei commenti di `tables.rs`. `Instruction.original_function` identifica la routine di emissione; le strutture dei selettori conservano indirizzi e/o righe aggiuntivi per ricostruire la decisione.

## Riesame dei 13 ID precedentemente UNKNOWN

| ID | Ruolo proposto / risultato | Stato attuale |
|---|---|---|
| `0x00` | Nessun caso nel dispatcher; non dimostrato se sia un tag riservato o un sentinel | UNKNOWN |
| `0x1B`, `0x1C` | Non emessi; coincidenze con ID di operatori nel parser non provano il significato AST | UNKNOWN |
| `0x2E` | Ramo return-only; nessun costruttore identificato in questa ricerca | UNKNOWN |
| `0x3C` | ArrayConstruct, lowering verso builtin9 `__vbaAryConstruct2` | INFERRED, HIGH |
| `0x3D` | ArrayDestruct, lowering verso builtin0xC `__vbaAryDestruct` | INFERRED, HIGH |
| `0x40` | FixedStringConstruct, lowering verso builtin0x9B `__vbaFixstrConstruct` | INFERRED, HIGH |
| `0x5B` | GosubCleanup, lowering verso builtin0x140 `__vbaGosubFree` | INFERRED, HIGH |
| `0x62` | PreparedValueWrapper: costruttore e figlio recuperati, attribuzione linguistica irrisolta | UNKNOWN, struttura OBSERVED |
| `0x64` | Il clonatore segue tre campi node[4..6], ma il visitor generico lo tratta come leaf; significato irrisolto | UNKNOWN |
| `0x70` | NamedArgument: nome risolto contro i parametri e figlio usato come valore | INFERRED, HIGH |
| `0x71` | ArgumentBindingMarker: copia tipo, conserva figlio, altera il binding; ipotesi parentesi/forced-value non confermata | INFERRED, MEDIUM |
| `0x73` | ProcedureContextWord: costruito durante preparazione del contesto e produce0x266+u16; ruolo preciso irrisolto | UNKNOWN, struttura OBSERVED |

Sei classificazioni nuove portano i ruoli identificati o inferiti a **109/116, 94,0%**. Restano sette UNKNOWN; due hanno ora una struttura più chiara. Questo conteggio non trasforma le inferenze in significati direttamente provati.

Le quattro associazioni runtime sono verificabili con `verification/verify_builtin_names.rs`: legge i puntatori dai record originali e confronta le stringhe. `FUN_0faedced` costruisce una chiamata0x61 con callee0x60 per0x3C/0x3D/0x40;0x5B usa `FUN_0fb8ebec` → `FUN_0faee05d` → `FUN_0faee1f5`. Eventuali conversioni del risultato restano a `FUN_0faee137`. La vecchia descrizione "lowering verso0x60" indicava solo il callee ed era incompleta.

Per ridurre gli ultimi dubbi servirebbe una traccia dei nodi prima e dopo binding/lowering del compilatore Windows della stessa versione. DLL6.0.0.8169 e DBG sono già disponibili localmente; le stringhe DBG esaminate non forniscono un enum AST privato. Campioni VBA minimali dovrebbero isolare argomenti nominati, argomenti fra parentesi, valori preparati e contesto di procedura. Per0x00/0x1B/0x1C/0x2E/0x64, una ricerca di xref completa sul binario o una traccia dei costruttori può anche stabilire che alcuni tag siano riservati o non prodotti in questa build: non va presunto.

## Verifica riproducibile

I test ordinari sono autonomi. I confronti opzionali usano i sorgenti precedenti in `../research/sources/backend/`:

```sh
sh verification/run.sh
```

Il backend di riferimento è impostato su `rust_enabled=false`, oppure si eseguono direttamente i blocchi originali della VM. Si confrontano byte, operandi e, dove applicabile, ritorno, offset, stack e stato. I fixture dei helper sono indicati nei sorgenti e nei messaggi dei test.

| Gruppo | Confronti riusciti |
|---|---:|
| Espressioni | 2340 |
| Chiamate late-bound | 3072 |
| Stream conversioni | 4855 |
| I/O builtin | 84 |
| Chiamate member, con helper risolti da fixture | 2160 |
| Selettori di indirizzamento | 5760 |
| Selettore conversioni | 220 |
| Ottimizzazione costanti | 24 |
| Switch assegnazione `0x2D`, con slot da fixture | 180 |
| Selettori member | 74 |
| Code native member, con helper da fixture | 4 |
| ReDim, con pool/puntatori da fixture | 96 |
| Get/Put, con pool da fixture | 36 |
| Proprietà | 400 |
| Assegnazioni indicizzate | 144 |
| Assegnazioni con retagging | 48 |
| Nodi di assegnazione condivisi e RHS floating | 36 |
| Riferimenti alle variabili, percorso compatto | 2592 |
| Prefissi di riferimento | 15 |
| Firma di riferimento, percorso FAC35B5 | 20 |
| Conversione Variant → tipo18, opcode436 | 5 |
| Riferimenti, percorso completo con classi/default | 1836 |
| Contesto oggetto/ambiente e riferimenti nominati | 384 |
| Composizione dei riferimenti | 96 |

Totale: **24.481 confronti**, oltre a **37 test Cargo**. La griglia sintetica delle conversioni mantiene 95 errori espliciti; il test del selettore di conversione esclude 22 combinazioni non risolte. Non sono casi superati e non sono inclusi nel totale. Questi numeri non misurano la percentuale di copertura del compilatore.

I quattro test Cargo aggiuntivi verificano il lowering noto e la sua emissione con firme da fixture; i record builtin sono confrontati separatamente con lo snapshot. I test di riferimento completi coprono anche classi/default sintetici: equivalenza numerica non significa che ogni combinazione sia prodotta da un programma VBA valido. L'oracolo è il Rust fornito che interpreta le istruzioni originali: conserva i limiti di quella ricostruzione.

## Parti ancora necessarie per la fedeltà completa

- Identificare semanticamente i 7 ID UNKNOWN e consolidare i nomi INFERRED analizzando costruttori e lowering.
- Ricostruire il binder e l’allocazione dei pool: slot, riferimenti e informazioni di simboli/firme sono ancora input risolti. La selezione dei descrittori e i loro effetti di emissione sono ora nel generatore.
- Estendere il lowering oltre i quattro builtin recuperati e valutare i domini rifiutati: indici fuori dalle tabelle recuperate, fallback che usano puntatori come modalità/opcode e combinazioni prive di evidenza sufficiente. Non sono inclusi fra i casi verificati.
- Implementare emissione degli statement, etichette, salti, backpatching e strutture della procedura/modulo.
- Confrontare il risultato con compilazioni differenziali del compilatore Microsoft, oltre al riferimento dello snapshot.

Alcuni tag intermedi condividono un ramo return-only del dispatcher. Il codice riproduce quel ritorno, ma non li considera costrutti VBA già compilabili: prima devono subire il lowering originale. Per i fallback nativi che troncano puntatori non ricostruiti, come alcuni descrittori di ReDim, restituisce `Unresolved`; non converte un indice dell'arena in un puntatore inventato.

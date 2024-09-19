# Simplesso

## Comandi

- Compilare: `cargo build`
- Eseguire: `./target/debug/simplesso ./example_data/test1`
- Compilare ed eseguire: `cargo run -- ./example_data/test1`

## Che cosa fa il programma

Il programma prende come argomento il path di una cartella contenente i file di input `A.txt`, `b.txt` e `c.txt` e stampa la soluzione del problema di programmazione lineare associato.

Il problema di programmazione lineare è il seguente:

```text
⎰ min b'*y
| y'*A = c 
⎱ y >= 0
```

Nel caso in cui la soluzione ottima esista essa sarà anche la soluzione ottima di:

```text
⎰ max c'*x
⎱ A*x <= b
```

ed il programma stamperà anche una x che soddisfa le condizioni di ottimalità.

## Formato dei file di input

Essi contengono delle matrici scritte per riga, con le righe separate da un carattere di newline e gli elementi di una riga separati da una virgola.

Ex. A.txt:

```text
1,0
0,1
-1,0
0,-1
```

equivale al matlab:

```matlab
A = [1,0;
     0,1;
    -1,0;
     0,-1];
```

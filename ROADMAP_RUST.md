# CleanLauncher - Roadmap & Rust Migration Sprints

La seguente roadmap definisce il percorso di migrazione per trasformare CleanLauncher in un'applicazione basata prevalentemente su Rust, mantenendo in C++ esclusivamente le componenti legate a Qt e all'interfaccia utente (UI). Questo garantisce un design più strutturato, sicuro (memory safety) ed efficiente.

## Obiettivo Principale

- **C++**: Solo Presentation Layer (Qt Widgets/QML, binding UI).
- **Rust (crates/)**: Core logic, networking, gestione file, parsing configurazioni, business logic, autenticazione e integrazioni di sistema.

---

## Roadmap e Sprints

### Sprint 1: Consolidamento del Core Rust e FFI

**Obiettivi:**

- Stabilire un'interfaccia solida tra C++ e Rust usando CXX o cbindgen.
- Migrare il parsing delle configurazioni base da C++ a Rust.
- **Task:**
  - Setup di `cxx` crate per binding bidirezionali sicuri.
  - Creazione di una struct condivisa per lo stato dell'applicazione.
  - Spostamento del log system in Rust (`tracing` o `log` crate) esponendo le API a C++.

### Sprint 2: Migrazione Networking e Autenticazione

**Obiettivi:**

- Sostituire le chiamate di rete (precedentemente in QtNetwork o libcurl) con `reqwest` in Rust.
- Migrare la logica di login, token refresh e gestione sessioni.
- **Task:**
  - Implementare client HTTP asincrono in Rust (tokio + reqwest).
  - Implementare l'autenticazione (OAuth/Microsoft Auth) in Rust.
  - Binding dei risultati di rete verso i ViewModel in C++.

### Sprint 3: Gestione File, Asset e Download Manager

**Obiettivi:**

- Riscrivere il sistema di download degli asset del gioco e della gestione file (I/O) in Rust.
- Ottimizzare download paralleli.
- **Task:**
  - Creazione di un Download Manager asincrono in Rust.
  - Validazione hash (SHA1/SHA256) per i file scaricati tramite `ring` o `sha2`.
  - Notifica del progresso di download tramite callback FFI al C++ (per le progressBar).

### Sprint 4: Lancio Processi e Gestione Ambiente

**Obiettivi:**

- Generazione degli argomenti di lancio e gestione del processo child del gioco.
- **Task:**
  - Parsing delle rule di lancio JSON in Rust.
  - Costruzione dell'environment (Java path, librerie native).
  - Esecuzione del processo e cattura di stdout/stderr da Rust (`std::process::Command` o `tokio::process`).

### Sprint 5: Testing, Memory Profiling e Sicurezza

**Obiettivi:**

- Integrazione di tool di profilazione e verifica di memory safety per Rust.
- **Task:**
  - Configurazione di **Miri** (l'alternativa/equivalente a Valgrind per intercettare Undefined Behavior in Rust) nei test.
  - Setup di `cargo-valgrind` per il memory leak detection del codice nativo.
  - Scrittura di unit test completi per i nuovi moduli in `crates/`.

### Sprint 6: Pulizia C++ e Refactoring UI

**Obiettivi:**

- Rimozione del codice C++ ormai obsoleto.
- Refactoring dei controller UI per essere "thin client" della libreria Rust.
- **Task:**
  - Eliminazione di vecchie classi manager C++.
  - Verifica della corretta separazione dei layer MVC/MVVM (C++ View, Rust Model/Controller).
  - Pulizia finale e ottimizzazione dei binari.

---

## Tooling: Alternative a Valgrind per Rust

Essendo Rust memory-safe, molti errori tipici del C++ sono prevenuti a compile-time. Tuttavia, per il codice `unsafe` (come FFI con C++) o per rilevare memory leak, implementeremo:

1. **Miri**: Interprete Rust per l'individuazione di Undefined Behavior e data race. Eseguibile tramite `cargo miri test`.
2. **cargo-valgrind**: Per eseguire i binari Rust con Valgrind e tracciare eventuali leak provenienti dalle interfacce C.

L'uso di Miri sarà integrato nelle pipeline di Continuous Integration per ogni PR che tocca la cartella `crates/`.

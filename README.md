# MarsIT-Rust
## Hoe het project lokaal draaien?

Eerst zal je nog wat dingen moeten installeren en initialiseren alvorens je het project kan pullen en dat je de api lokaal kan draaien. 


 ### - Ga naar de [website](https://www.rust-lang.org/tools/install) van Rust en download hierbij de nieuwste versie van Rust
Tijdens de installatie moet je er zeker voor zorgen de je nightly kiest als standaard Rust repo.
### - Pas je omgevingsvariabelen aan:

 - Typ in de zoekbalk: omgevingsvariabelen
 - klik op **Omgevingsvariabelen voor uw account bewerken**
 - dubbelklik dan op path en voeg het pad toe naar de **bin** folder in de
   **.cargo** folder deze is standaard geïnstaleerd onder het pad **C:\Users\gebruiker\\.cargo\bin**
 - Sla dit alles op en een herstart de machine om er voor te zorgen dat alle wijzigingen actief zijn

### - Zorg ook zeker voor dat de C++ build tools geïnstalleerd zijn op je machine
### - Nu kan je over gaan tot het clonen van het project
### - Eens deze clone succesvol is gelukt, kan je het project runnen door volgende commando's uit te voeren:
	cargo update
	cargo build
	cargo run

## Rust editten in IntelliJ

In IntelliJ kan je ook zeer gemakkelijk rust editten en runnen. Hiervoor heb je maar 2 plugins nodig, deze plugins zijn:

 - De [Rust](https://plugins.jetbrains.com/plugin/8182-rust) plugin van Jetbrains
 - De [Toml](https://plugins.jetbrains.com/plugin/8195-toml) plugin van Jetbrains

Hierna moet je nog 1 iets checken: ga naar Settings > Languages & Frameworks > Rust en bekijk als de Toolchain location correct ingesteld staat op de **.cargo\bin folder**.
Best bekijk je ook nog eens als de standard library correct staat ingesteld.

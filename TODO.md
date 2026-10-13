# Projekt _4gen - Lista Zadań (TODO)

## ZNANE BŁĘDY WYMAGAJĄCE SZYBKIEJ KOREKTY

### WERSJA 2026-42AUT-2 16:00:33.078|04:41

Błędy związane z Tabelami:

- [ ] **A.1** Brak możliwości przewijania tabeli w pionie za pomocą gestów dotykowych (np. palcem) w obszarze kolumn, które nie są przypięte. Obecnie, aby przewinąć tabelę w pionie, trzeba przypiąć dowolną kolumnę i użyć obszaru przypiętego do scrollowania. W obszarze kolumn nieprzypiętych działa wyłącznie przewijanie poziome, co jest nielogiczne i utrudnia nawigację. *(Zgłoszone dla: WASM na Androidzie)*.
- [ ] **A.2** Wiersze kolumn przypiętych są nieznacznie obniżone względem kolumn nieprzypiętych (rozjazd w osi Y).
- [ ] **A.3** Brak obsługi przepełnienia (overflow) na pasku narzędzi (toolbar). Należy również dodać połączony modyfikator `Ctrl+Shift` w tym samym stylu wizualnym co pozostałe klawisze funkcyjne.
- [ ] **A.4** Obsługa przeciągania elementów palcem (Drag & Drop na urządzeniach mobilnych) działa bardzo kapryśnie i wymaga znacznej poprawy płynności oraz czułości.
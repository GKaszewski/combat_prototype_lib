# Combat prototype

- hex grid (trzeba napisac jakas abstrakcje na grida, tak by mechaniki mogly miec wylane jaki grid jest, ale docelowo ma byc hex)

**Unit**

- ID (jakis string pewnie: 'archer')
- Level
- Experience
- Initiative
- Defense
- Attack
- Health
- Range
- Movement points

**Gameplay**\
Gracz przed walka ustawia sobie layout armii.
Na każda ture przeznaczony jest określony budżet 'ruchów' (basically ile jednostek może wykonać akcję w danej turze).

Kolejność zależy od inicjatywy danej jednostki, im wyższa inicjatywa to ta jednostka ma pierwszeństwo, jeśli występują obie jednostki o tej samej wartości inicjatywy to rusza ta co ma większą wartość ataku, a jak to mają to samo to obrony, a jak to to zdrowia, w innym przypadku leci pierwsza z brzegu.

Atak 'od tyłu' zadaje dodatkowe 5% obrażen.
Jeśli jednostka jest w trybie obrony to skipuje ture, jak wystąpi atak to obrażenia sa obniżone o 5% (podstawowo, tutaj ulepszenia wchodza na pełnej i można sobie zwiększyć modyfikator)

Warunkiem do wygrania jest pokonanie wszystkich jednostek/doprowadzenie do poddania sie przeciwnika. (Może jakaś mechanika morali????)

### Notes

- use petgraph instead of my Grid trait
- use Command pattern for actions

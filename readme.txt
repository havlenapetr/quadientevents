/*
 * Copyright (C) 2017 Petr Havlena (havlenapetr@gmail.com)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

 jedna se o muj prvni pokus v jazyce RUST (chtel jsem ho uz delsi dobu zkusit), tak se
 omlouvam pokud jsem neco resil ne zcela RUSTove, algoritmus si poradi se zadanim na
 PC DELL OptiPlex 7040 neco mezi 0.4sec - 0.7sec

root
  |
  |-- src
  |    |
  |    |-- map.rs
  |    |-- printer.rs
  |    |-- task.rs
  |    |-- utils.rs
  |    |-- algorithm.rs
  |    |-- main.rs
  |
  |-- tests
       |
       |-- tests.rs
       |-- task3x3.txt
       |-- task300x300.txt

map.rs
    nainicializuje se ze zadani a poskytuje zakladni fce pro pohyb v mape

printer.rs
    jednoduchy vykreslovac mapy do stdoutu, ma smysl jen pro mensi rozmery map

task.rs
    zodpovedny za komunikaci se serverem, tzn. ziskani zadani a poslani reseni

utils.rs
    jednoduche util metody pro mereni casu a otoceni souradnic pro "start" a "end" zadani

algorithm.rs
    jednoducha implementace A* algoritmu bez heuristicke fce, zohlednuje se jen zda-li
    se do daneho uzlu(bodu) jiz vstupovalo ci nikoliv pomoci hashsetu(visited_nodes)

main.rs
    obsahuje main fci, kde jsou hlavni includy, nacteni zadani, spusteni reseni,
    nasledne poslani a kontrola zjisteneho reseni

tests.rs
    par testu na fce mapy a algoritmu

task3x3.txt
    jednoduche zadani 3x3 pro testy

task300x300.txt
    zadani z ukolu 300x300 na mereni performance
